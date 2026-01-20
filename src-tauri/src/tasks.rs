use crate::api::fetch_usage;
use crate::config::load_config;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};

pub type UsageState = Arc<Mutex<Option<crate::api::UsageData>>>;
pub type ErrorState = Arc<Mutex<Option<String>>>;

pub struct UsageTask {
    handle: AppHandle,
    usage_state: UsageState,
    error_state: ErrorState,
}

impl UsageTask {
    pub fn new(handle: AppHandle, usage_state: UsageState, error_state: ErrorState) -> Self {
        Self {
            handle,
            usage_state,
            error_state,
        }
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(10)); // 首次10秒后执行

        loop {
            interval.tick().await;

            let config = match load_config() {
                Some(cfg) => cfg,
                None => {
                    *self.error_state.lock().await = Some("未配置 API 信息".to_string());
                    continue;
                }
            };

            if !crate::config::is_config_valid(&config) {
                *self.error_state.lock().await = Some("API 配置不完整".to_string());
                continue;
            }

            match fetch_usage(&config).await {
                Ok(usage) => {
                    *self.usage_state.lock().await = Some(usage.clone());
                    *self.error_state.lock().await = None;

                    // 更新托盘图标标题
                    if let Err(e) = self.update_tray_title(&usage) {
                        eprintln!("更新托盘标题失败: {}", e);
                    }

                    // 发送事件到前端
                    let _ = self.handle.emit("usage-update", usage);
                }
                Err(e) => {
                    *self.error_state.lock().await = Some(e.clone());
                    let _ = self.handle.emit("usage-error", e);
                }
            }

            // 根据配置调整间隔
            let new_interval = Duration::from_secs(config.refresh_interval);
            interval = tokio::time::interval(new_interval);
        }
    }

    fn update_tray_title(&self, usage: &crate::api::UsageData) -> Result<(), String> {
        let title = format!(
            "GLM: {}/{} ({}%)",
            crate::api::format_tokens(usage.used_tokens),
            crate::api::format_tokens(usage.total_tokens),
            usage.usage_percentage as u32
        );

        // 这里需要从 AppHandle 获取 tray 实例
        // 暂时通过事件发送到前端处理
        let _ = self.handle.emit("tray-title-update", title);
        Ok(())
    }
}

pub fn start_background_task(
    handle: AppHandle,
    usage_state: UsageState,
    error_state: ErrorState,
) {
    tauri::async_runtime::spawn(async move {
        let task = UsageTask::new(handle, usage_state, error_state);
        task.run().await;
    });
}
