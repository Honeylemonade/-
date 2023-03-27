#![cfg_attr(
all(not(debug_assertions), target_os = "macos"),
windows_subsystem = "windows"
)]

use mysql::*;
use mysql::prelude::*;
use urlencoding::encode;

#[tauri::command(rename_all = "snake_case")]
fn get_db_connect(end_point: &str, database: &str, user_name: &str, password: &str) -> bool {
    let url = format!("mysql://{}:{}@{}/{}", user_name, encode(password), end_point, database);
    println!("{}", url);
    let pool = Pool::new(&*url).expect("连接失败"); // 获取连接池
    pool.get_conn().unwrap(); // 获取链接
    return true;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_db_connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
