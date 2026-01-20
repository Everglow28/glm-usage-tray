use crate::config::ApiConfig;
use crate::{debug, warn, error};
use serde::{Deserialize, Serialize};

/// 完整的用量数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub code: u32,
    pub msg: String,
    pub data: UsageLimits,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLimits {
    pub limits: Vec<LimitItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitItem {
    #[serde(rename = "type")]
    pub limit_type: String,
    pub unit: u32,
    pub number: u32,
    pub usage: u64,
    #[serde(rename = "currentValue")]
    pub current_value: u64,
    pub remaining: u64,
    pub percentage: f64,
    #[serde(rename = "usageDetails", default)]
    pub usage_details: Option<Vec<UsageDetail>>,
    #[serde(rename = "nextResetTime", default)]
    pub next_reset_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDetail {
    #[serde(rename = "modelCode")]
    pub model_code: String,
    pub usage: u64,
}


pub async fn fetch_usage(config: &ApiConfig) -> Result<UsageData, String> {
    debug!("开始获取用量数据...");
    debug!("Token: {}...", &config.token[..config.token.len().min(20)]);
    debug!("Organization: {}", config.organization);
    debug!("Project: {}", config.project);

    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| format!("创建客户端失败: {}", e))?;

    let response = client
        .get("https://bigmodel.cn/api/monitor/usage/quota/limit")
        .header("authorization", &config.token)
        .header("bigmodel-organization", &config.organization)
        .header("bigmodel-project", &config.project)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = response.status();
    let body = response.text().await.map_err(|e| format!("读取响应失败: {}", e))?;

    debug!("API 响应状态: {}", status);
    debug!("API 原始响应: {}", body);

    if !status.is_success() {
        if status.as_u16() == 401 {
            warn!("Token 已过期或无效，请更新配置");
            return Err("Token 已过期或无效，请更新配置".to_string());
        }
        error!("API 错误 ({}): {}", status.as_u16(), body);
        return Err(format!("API 错误 ({}): {}", status.as_u16(), body));
    }

    // 尝试解析 JSON 响应
    let api_response: UsageData =
        serde_json::from_str(&body).map_err(|e| format!("解析响应失败: {} | 响应内容: {}", e, body))?;

    debug!("解析后数据: {:?}", api_response);

    Ok(api_response)
}

// 格式化 token 数量为人类可读形式
pub fn format_tokens(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{:.1}M", tokens as f64 / 1_000_000.0)
    } else if tokens >= 1_000 {
        format!("{:.1}K", tokens as f64 / 1_000.0)
    } else {
        tokens.to_string()
    }
}

