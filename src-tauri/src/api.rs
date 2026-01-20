use crate::config::ApiConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub total_tokens: u64,
    pub used_tokens: u64,
    pub remaining_tokens: u64,
    pub usage_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    #[serde(rename = "data")]
    data: ApiData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiData {
    #[serde(rename = "totalTokens")]
    total_tokens: Option<u64>,
    #[serde(rename = "usedTokens")]
    used_tokens: Option<u64>,
    #[serde(rename = "remainingTokens")]
    remaining_tokens: Option<u64>,
    // 根据实际 API 响应调整字段
}

pub async fn fetch_usage(config: &ApiConfig) -> Result<UsageData, String> {
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

    if !status.is_success() {
        if status.as_u16() == 401 {
            return Err("Token 已过期或无效，请更新配置".to_string());
        }
        return Err(format!("API 错误 ({}): {}", status.as_u16(), body));
    }

    // 尝试解析 JSON 响应
    let api_response: ApiResponse =
        serde_json::from_str(&body).map_err(|e| format!("解析响应失败: {} | 响应内容: {}", e, body))?;

    let total = api_response.data.total_tokens.unwrap_or(0);
    let used = api_response.data.used_tokens.unwrap_or(0);
    let remaining = api_response.data.remaining_tokens.unwrap_or(0);

    // 计算使用百分比
    let usage_percentage = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    Ok(UsageData {
        total_tokens: total,
        used_tokens: used,
        remaining_tokens: remaining,
        usage_percentage,
    })
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
