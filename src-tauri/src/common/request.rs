use lazy_static::lazy_static;
use log::{debug, error};
use std::sync::{RwLock};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;
use rand::{random, Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json::{Map, Number, Value};
use rand::prelude::SliceRandom;
use reqwest::header::HeaderMap;
use tauri::{Window};
use tauri::async_runtime::block_on;
// current mod：定时提交tasks（多个task，一个模型一个 task）
// history mod：上次提交的Tasks全部返回后（多个task，一个模型一个 task），提交下一批tasks

// 需要能拿到 tasks 的返回结果[存到一个 map 里，一个 模型一个 entry]
// 需要能暂停 tasks 的提交【循环里/提交前 做一个判断，查看当前状态是不是 running，还是 stoped】

// models state T/F
// 历史上数导入进度 pogress
// 运行状态 RUNNING_STATE: RUNNING/STOP
// 运行状态 RUNNING_MODE: CURRENT/HISTORY

lazy_static! {
    static ref STATE_MAP: RwLock<HashMap<String,String>> = RwLock::new(HashMap::new());
    static ref SHOULD_RUN:AtomicBool =AtomicBool::new(false);
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum RunningMode {
    current,
    history,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct TimeConf {
    time_interval: u64,
    start_time: u64,
    end_time: u64,
    time_step: u64,
}

pub fn test_backend_connect(backend_end_point: &str, window: Window) -> bool {
    // 超时时间为1s
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(Duration::from_secs(1))
        .build().unwrap();

    match client.get(format!("http://{}/v1/health", backend_end_point)).send() {
        Ok(_) => true,
        Err(e) => {
            error!("{}", e);
            window.emit("log", "后端连接失败").unwrap();
            return false;
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn run(running_mode: RunningMode, time_conf: TimeConf, backend_end_point: String,
           model_confs_json_str: String, window: Window) {
    debug!("Start run");
    let model_confs: Value = serde_json::from_str(&model_confs_json_str).expect("JSON 解析失败");
    SHOULD_RUN.store(true, Ordering::Relaxed);

    match running_mode {
        RunningMode::current => {
            thread::spawn(move || {
                current_run(time_conf.time_interval, backend_end_point, model_confs, window);
            });
        }
        RunningMode::history => {
            thread::spawn(move || {
                history_run(time_conf.start_time, time_conf.end_time,
                            time_conf.time_step, backend_end_point, model_confs, window);
            });
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn stop(window: Window) {
    SHOULD_RUN.store(false, Ordering::Relaxed);
    debug!("stop!");
    window.emit("log", "任务停止").unwrap();
}


fn current_run(time_interval: u64, backend_end_point: String, model_confs: Value, window: Window) {
    debug!("current_run");
    let mut last_exec_time = Instant::now();
    loop {
        // 等待减去上一次任务执行的实际时间
        let duration_since_last_exec = Instant::now().duration_since(last_exec_time);


        if duration_since_last_exec >= Duration::from_millis(time_interval) {
            // Do nothing 立刻执行
        } else {
            //等一段时间
            let a = Duration::from_millis(time_interval) - duration_since_last_exec;
            thread::sleep(a);
        }


        // 若被暂停则停止
        if !SHOULD_RUN.load(Ordering::Acquire) {
            debug!("current_run break");
            break;
        } else {
            last_exec_time = Instant::now();
            do_task(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64, &backend_end_point, model_confs.clone(), window.clone());
        }
    }
}

fn history_run(start_time: u64, end_time: u64, time_step: u64, backend_end_point: String,
               model_confs: Value, window: Window) {
    let mut current_timestamp = start_time;

    while current_timestamp <= end_time {
        current_timestamp += time_step;


        let progress = (current_timestamp - start_time) as f64 / (end_time - start_time) as f64;
        debug!("当前进度:{}",progress);
        window.emit("progress", progress).unwrap();
        // 若被暂停则停止
        if !SHOULD_RUN.load(Ordering::Acquire) {
            debug!("current_run break");
            break;
        } else {
            do_task(current_timestamp, &backend_end_point, model_confs.clone(), window.clone());
        }
    }
}


fn do_task(timestamp: u64, backend_end_point: &str, model_confs: Value, window: Window) {
    window.emit("log", format!("开始执行任务")).unwrap();
    // 针对每一个模型配置生成 item
    for model_conf in model_confs.as_array().unwrap() {
        let model_name = model_conf.as_object().unwrap().get("modelName").unwrap();
        let object_name_list = model_conf.as_object().unwrap().get("objectNameList").unwrap();
        let object_name_list: Vec<&str> = object_name_list.as_array().unwrap().iter()
            .map(|e| { e.as_str().unwrap() })
            .collect();

        let mut message_list: Vec<Value> = Vec::new();

        // 每一个实例都随机构建一次
        for object_name in object_name_list {
            let mut properties: Map<String, Value> = Map::new();
            let property_confs = model_conf.as_object().unwrap().get("propertyConfs").unwrap();
            for property_conf in property_confs.as_array().unwrap() {
                match build_value(property_conf) {
                    None => {}
                    Some((key, value)) => {
                        properties.insert(String::from(key), value);
                    }
                }
            }

            let mut message_list_item: Map<String, Value> = Map::new();
            message_list_item.insert(String::from("modelName"), model_name.clone());
            message_list_item.insert(String::from("objectName"), Value::String(String::from(object_name)));
            message_list_item.insert(String::from("properties"), Value::Object(properties.clone()));

            message_list.push(Value::Object(message_list_item));
        }

        //body_map.insert(String::from("messageList"), Value::from(message_list.clone()));

        // post 请求要创建client
        let client = reqwest::Client::new();

        // 组装header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut data = HashMap::new();
        data.insert("messageList", message_list);

        // 发起post请求并返回
        let response = client.post(format!("http://{}/v1/object-manager/messages", backend_end_point))
            .headers(headers)
            .json(&data)
            .send();

        match block_on(response) {
            Ok(_) => {
                println!("发送成功");
                window.emit("log", format!("写入模型[{}]数据成功", model_name.clone())).unwrap();
                window.emit("log", &serde_json::to_string(&data).unwrap()).unwrap();
            }
            Err(e) => {
                println!("发送失败:{}", e);
                window.emit("log", format!("写入模型[{}]数据失败", model_name.clone())).unwrap();
            }
        }
    }
}


fn build_value(property_conf: &Value) -> Option<(&str, Value)> {
    if !property_conf.get("enable").unwrap().as_bool().unwrap() {
        return Option::None;
    }
    let key = property_conf.get("name").unwrap().as_str().unwrap();
    let data_type = property_conf.get("data_type").unwrap().as_str().unwrap();
    let is_random = property_conf.get("is_random").unwrap_or(&Value::Bool(false)).as_bool().unwrap();
    let lower_bound = property_conf.get("lower_bound").unwrap_or(&Value::Number(Number::from(-100000))).as_f64().unwrap();
    let upper_bound = property_conf.get("upper_bound").unwrap_or(&Value::Number(Number::from(100000))).as_f64().unwrap();
    let dp = property_conf.get("dp").unwrap_or(&Value::Number(Number::from(2))).as_i64().unwrap();

    // 常量
    if !is_random {
        return Some((key, property_conf.get("const").unwrap().clone()));
    }

    // 随机
    let value: Value = match data_type {
        "string" => {
            match property_conf.get("const") {
                None => {
                    Value::String(generate_random_string())
                }
                Some(_) => {
                    // const 中的随机
                    let str_values: Vec<&str> = property_conf.get("const").unwrap().as_array()
                        .unwrap()
                        .iter()
                        .map(|e| { return e.as_str().unwrap(); })
                        .collect();
                    let result = random_string(str_values);
                    Value::String(String::from(result))
                }
            }
        }
        "double" => {
            // TODO 小数位数指定
            let random_number = thread_rng().gen_range(lower_bound..upper_bound);
            let multiplier = 10.0_f64.powi(dp as i32);
            let r_num: f64 = (random_number * multiplier).round() / multiplier;

            Value::Number(Number::from_f64(r_num).unwrap())
        }
        "long" => {
            let r_num = thread_rng().gen_range(lower_bound..upper_bound);
            Value::Number(Number::from(format!("{:.0}", r_num).parse::<i64>().unwrap()))
        }
        "boolean" => {
            Value::Bool(random())
        }
        _ => { Value::Number(Number::from(0)) }
    };

    Some((key.clone(), value))
}

fn generate_random_string() -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

fn random_string(str_values: Vec<&str>) -> &str {
    let mut rng = thread_rng();
    //let strings = &str_values[..];;
    return str_values.choose(&mut rng).unwrap();
}
