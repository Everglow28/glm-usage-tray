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

/// 限制类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitType {
    TimeLimit,
    TokensLimit,
    Unknown,
}

impl LimitType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "TIME_LIMIT" => LimitType::TimeLimit,
            "TOKENS_LIMIT" => LimitType::TokensLimit,
            _ => LimitType::Unknown,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            LimitType::TimeLimit => "时间限额",
            LimitType::TokensLimit => "Token 限额",
            LimitType::Unknown => "未知限额",
        }
    }
}

impl LimitItem {
    pub fn get_limit_type(&self) -> LimitType {
        LimitType::from_str(&self.limit_type)
    }

    /// 获取重置时间描述
    pub fn get_reset_time_text(&self) -> String {
        if let Some(reset_time) = self.next_reset_time {
            // 将时间戳转换为可读格式
            let datetime = format_timestamp(reset_time);
            format!("重置时间: {}", datetime)
        } else if self.limit_type == "TIME_LIMIT" {
            // 时间限制：每 N 单位重置
            match self.unit {
                1 => "每小时重置".to_string(),
                5 => "每5小时重置".to_string(),
                24 => "每天重置".to_string(),
                _ => format!("每{}小时重置", self.unit),
            }
        } else {
            "重置时间: 未知".to_string()
        }
    }
}

/// 将时间戳转换为可读格式
fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, UNIX_EPOCH};

    // 检查是秒还是毫秒
    let secs = if timestamp > 1_000_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    };

    // 计算 UTC 时间
    if UNIX_EPOCH.checked_add(Duration::from_secs(secs)).is_some() {
        // 获取自纪元以来的秒数，计算本地时间偏移
        // 这里简化处理，假设 UTC+8（中国时区）
        let secs_since_epoch = secs + 8 * 3600;

        let days = secs_since_epoch / 86400;
        let remaining = secs_since_epoch % 86400;
        let hours = remaining / 3600;
        let minutes = (remaining % 3600) / 60;

        // 简化：从1970-01-01开始计算天数到日期
        // 这是一个简化版本
        let approx_days = days as i64;
        let approx_year = 1970 + approx_days / 365;
        let approx_day_of_year = (approx_days % 365) as u32;
        let approx_month = approx_day_of_year / 30 + 1;
        let approx_day = (approx_day_of_year % 30) + 1;

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}",
            approx_year, approx_month, approx_day, hours, minutes
        )
    } else {
        format!("{}", timestamp)
    }
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

// 格式化数字
pub fn format_number(num: u64) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}
