use crate::api::{fetch_usage, format_tokens, UsageData};
use crate::config::{load_config, save_config, ApiConfig};
use tauri::State;

pub type UsageState = std::sync::Arc<tokio::sync::Mutex<Option<UsageData>>>;
pub type ErrorState = std::sync::Arc<tokio::sync::Mutex<Option<String>>>;

#[tauri::command]
pub fn get_config() -> Option<ApiConfig> {
    load_config()
}

#[tauri::command]
pub fn save_config_cmd(config: ApiConfig) -> Result<(), String> {
    save_config(&config)
}

#[tauri::command]
pub fn test_connection(config: ApiConfig) -> Result<UsageData, String> {
    if config.token.is_empty() || config.organization.is_empty() || config.project.is_empty() {
        return Err("请填写完整的配置信息".to_string());
    }

    // 使用 blocking task 执行异步调用
    tauri::async_runtime::block_on(async { fetch_usage(&config).await })
}

#[tauri::command]
pub fn get_current_usage(
    usage_state: State<'_, UsageState>,
) -> Result<Option<UsageData>, String> {
    let state = usage_state.lock().map_err(|e| e.to_string())?;
    Ok((*state).clone())
}

#[tauri::command]
pub fn get_current_error(error_state: State<'_, ErrorState>) -> Result<Option<String>, String> {
    let state = error_state.lock().map_err(|e| e.to_string())?;
    Ok((*state).clone())
}

#[tauri::command]
pub fn hide_window() -> Result<(), String> {
    tauri::async_runtime::block_on(async {
        if let Some(window) = tauri::webview_window::WebviewWindow::get_by_label("config") {
            let _ = window.hide();
        }
        Ok(())
    })
}
