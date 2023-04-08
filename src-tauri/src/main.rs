#![cfg_attr(
all(not(debug_assertions), target_os = "macos"),
windows_subsystem = "windows"
)]

use std::io::Write;
use chrono::Local;

mod common;

use log::LevelFilter;

use tauri::Manager;
use env_logger::Builder;

fn main() {
    Builder::from_default_env()
        .format(|buf, record| {
            let thread_id = {
                let thread = std::thread::current();
                format!("{:?}", thread.id())
            };
            writeln!(buf, "{} [{}] [{}] {}", Local::now(), thread_id, record.level(), record.args())
        })
        .filter(None, LevelFilter::Debug)
        .try_init().unwrap();

    tauri::Builder::default()
        .setup(|app| {
            //#[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.maximize().expect("最大化异常");
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![common::test_connect,
            common::db::get_model_list,
            common::db::get_object_list_by_model_name,
            common::db::get_model_property_def,
            common::request::run,
            common::request::stop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
