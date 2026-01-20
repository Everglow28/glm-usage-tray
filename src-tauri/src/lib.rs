mod api;
mod commands;
mod config;
mod tasks;
mod tray;

use commands::{ErrorState, UsageState};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let usage_state: UsageState = Arc::new(Mutex::new(None));
    let error_state: ErrorState = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(usage_state.clone())
        .manage(error_state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::save_config_cmd,
            commands::test_connection,
            commands::get_current_usage,
            commands::get_current_error,
            commands::hide_window,
        ])
        .setup(|app| {
            // 创建系统托盘
            tray::create_tray(app)?;

            // 启动后台任务
            let handle = app.handle().clone();
            tasks::start_background_task(handle, usage_state, error_state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
