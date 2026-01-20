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
        // 首次加载配置获取初始间隔
        let initial_interval = load_config()
            .map(|c| c.refresh_interval)
            .unwrap_or(60);

        let mut interval = tokio::time::interval(Duration::from_secs(initial_interval));
        // 消耗第一次立即触发，使周期从现在开始计算
        interval.tick().await;

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
        }
    }

    fn update_tray_title(&self, usage: &crate::api::UsageData) -> Result<(), String> {
        // 从 limits 数组中找到 TOKENS_LIMIT 类型
        if let Some(token_limit) = usage.data.limits.iter()
            .find(|item| item.limit_type == "TOKENS_LIMIT")
        {
            let title = format!(
                "GLM: {}/{} ({}%)",
                crate::api::format_tokens(token_limit.current_value),
                crate::api::format_tokens(token_limit.usage),
                token_limit.percentage as u32
            );
            let _ = self.handle.emit("tray-title-update", title);
        } else {
            // 如果没找到 TOKENS_LIMIT，显示通用信息
            let title = format!("GLM: {} 限额", usage.data.limits.len());
            let _ = self.handle.emit("tray-title-update", title);
        }

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
