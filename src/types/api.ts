// GLM API 响应类型定义

export interface UsageDetail {
  modelCode: string;  // Rust serde: model_code -> modelCode
  usage: number;
}

export interface Limit {
  type: string;           // "TIME_LIMIT" | "TOKENS_LIMIT"
  percentage: number;
  currentValue: number;   // Rust serde: current_value -> currentValue
  usage: number;
  remaining: number;
  usageDetails?: UsageDetail[];  // Rust serde: usage_details -> usageDetails
  nextResetTime?: string;  // Rust serde: next_reset_time -> nextResetTime
}

export interface UsageData {
  success: true;
  data: {
    limits: Limit[];
  };
}

export interface UsageError {
  success: false;
  code: string;
  message: string;
}

export type UsageResponse = UsageData | UsageError;

export interface AppConfig {
  token: string;
  organization: string;
  project: string;
  refresh_interval: number;
}
