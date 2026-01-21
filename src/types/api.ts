// GLM API 响应类型定义

export interface UsageDetail {
  model_code: string;
  usage: number;
}

export interface Limit {
  type: string;           // "TIME_LIMIT" | "TOKENS_LIMIT"
  percentage: number;
  current_value: number;
  usage: number;
  remaining: number;
  usage_details?: UsageDetail[];
  next_reset_time?: string;
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
