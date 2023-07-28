use core::time;
use std::collections::HashMap;
use urlencoding::encode;
use log::{debug, info};
use sqlx::{FromRow, MySql, Pool, Row};
use sqlx::mysql::{MySqlPoolOptions};
use tauri::async_runtime::block_on;
use lazy_static::*;
use std::sync::Mutex;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Window;
use tokio::time::timeout;

lazy_static! {
    //  TODO使用读写锁
    static ref POOL_MAP: Mutex<HashMap<String,Pool<MySql>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Model {
    iid: String,
    name: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Objet {
    display_name: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Property {
    display_name: String,
    name: String,
    data_type: String,
}


pub fn test_db_connect(db_end_point: &str,
                       database: &str,
                       user_name: &str,
                       password: &str,
                       window: Window) -> bool {
    let url = format!("mysql://{}:{}@{}/{}", user_name, encode(password), db_end_point, database);
    info!("url={}",url);

    let pool_future = MySqlPoolOptions::new().acquire_timeout(time::Duration::from_secs(2))
        .connect(&url);
    //let timeout_future = timeout(std::time::Duration::from_secs(1), pool_future);
    let pool = match block_on(pool_future) {
        Ok(pool) => pool,
        Err(_e) => {
            window.emit("log", "数据库连接失败").unwrap();
            return false;
        }
    };
    info!("连接池创建成功");

    POOL_MAP.lock().unwrap().insert(String::from("pool"), pool);

    true
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_model_list() -> Result<Vec<Model>, String> {
    debug!("开始查询模型列表");
    let map = POOL_MAP.lock().unwrap();
    let pool = match map.get("pool") {
        None => { return Err(String::from("请检查连接")); }
        Some(p) => p
    };

    let result = block_on(sqlx::query(
        r#"SELECT iid,name,display_name FROM om_interfaces_info ORDER BY update_time DESC;"#)
        .fetch_all(pool)).unwrap();

    Ok(result.iter()
        .map(|row| {
            return Model {
                iid: row.get("iid"),
                name: row.get("name"),
                display_name: row.get("display_name"),
            };
        }).collect())
}


#[tauri::command(rename_all = "snake_case")]
pub fn get_object_list_by_model_name(model_name: &str) -> Result<Vec<Objet>, String> {
    debug!("开始实例列表");
    let map = POOL_MAP.lock().unwrap();
    let pool = match map.get("pool") {
        None => { return Err(String::from("请检查连接")); }
        Some(p) => p
    };

    let sql = format!("SELECT display_name, name FROM om_objects_info WHERE iid ='dtmi:{};1' ORDER BY update_time DESC;", model_name);

    let result = block_on(sqlx::query(
        &*sql)
        .fetch_all(pool)).unwrap();

    Ok(result.iter()
        .map(|row| {
            return Objet {
                name: row.get("name"),
                display_name: row.get("display_name"),
            };
        }).collect())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_model_property_def(model_name: &str) -> Result<Vec<Property>, String> {
    debug!("开始模型属性列表");
    let map = POOL_MAP.lock().unwrap();
    let pool = match map.get("pool") {
        None => { return Err(String::from("请检查连接")); }
        Some(p) => p
    };

    let sql = format!("SELECT display_name,name,data_schema FROM om_interfaces_property WHERE \
    iid='dtmi:{};1' AND `name`!='name' ORDER BY create_time ASC", model_name);

    let result = block_on(sqlx::query(
        &*sql)
        .fetch_all(pool)).unwrap();

    Ok(result.iter()
        .map(|row| {
            let v: Value = serde_json::from_str(row.get("data_schema")).unwrap();
            let data_type = v["type"].as_str().unwrap();

            return Property {
                name: row.get("name"),
                display_name: row.get("display_name"),
                data_type: String::from(data_type),
            };
        }).collect())
}




