pub mod db;
pub mod request;

use tauri::Window;
use db::test_db_connect;
use request::test_backend_connect;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum ConnectState {
    DbError,
    BackendError,
    Success,
}

#[tauri::command(rename_all = "snake_case")]
pub fn test_connect(backend_end_point: &str,
                    db_end_point: &str,
                    database: &str,
                    user_name: &str,
                    password: &str,
                    window: Window) -> ConnectState {
    let is_backend_connect = test_backend_connect(backend_end_point, window.clone());

    let is_db_connect = test_db_connect(db_end_point, database, user_name, password, window.clone());

    if !is_backend_connect {
        return ConnectState::BackendError;
    }
    if !is_db_connect {
        return ConnectState::DbError;
    }
    window.emit("log", "连接成功！").unwrap();
    return ConnectState::Success;
}
