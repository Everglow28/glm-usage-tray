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
    data: ApiData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiData {
    limits: Vec<LimitItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LimitItem {
    #[serde(rename = "type")]
    limit_type: String,
    usage: u64,
    #[serde(rename = "currentValue")]
    current_value: u64,
    remaining: u64,
    percentage: f64,
}

pub async fn fetch_usage(config: &ApiConfig) -> Result<UsageData, String> {
    println!("[DEBUG] 开始获取用量数据...");
    println!("[DEBUG] Token: {}...", &config.token[..config.token.len().min(20)]);
    println!("[DEBUG] Organization: {}", config.organization);
    println!("[DEBUG] Project: {}", config.project);

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

    println!("[DEBUG] API 响应状态: {}", status);
    println!("[DEBUG] API 原始响应: {}", body);

    if !status.is_success() {
        if status.as_u16() == 401 {
            return Err("Token 已过期或无效，请更新配置".to_string());
        }
        return Err(format!("API 错误 ({}): {}", status.as_u16(), body));
    }

    // 尝试解析 JSON 响应
    let api_response: ApiResponse =
        serde_json::from_str(&body).map_err(|e| format!("解析响应失败: {} | 响应内容: {}", e, body))?;

    println!("[DEBUG] 解析后数据: {:?}", api_response.data);

    // 从 limits 数组中找到 TOKENS_LIMIT 类型的条目
    let token_limit = api_response.data.limits
        .iter()
        .find(|item| item.limit_type == "TOKENS_LIMIT")
        .ok_or_else(|| format!("API 响应中未找到 TOKENS_LIMIT 类型: {:?}", api_response.data.limits))?;

    let total = token_limit.usage;
    let used = token_limit.current_value;
    let remaining = token_limit.remaining;
    let percentage = token_limit.percentage;

    println!("[DEBUG] total: {}, used: {}, remaining: {}, percentage: {}", total, used, remaining, percentage);

    Ok(UsageData {
        total_tokens: total,
        used_tokens: used,
        remaining_tokens: remaining,
        usage_percentage: percentage,
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
