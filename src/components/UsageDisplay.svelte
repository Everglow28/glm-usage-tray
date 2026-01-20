<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let usage: any;
  export let error: string | null;
  export let onConfig: () => void;

  let lastUpdate: Date | null = null;
  let limits: any[] = [];

  // 监听 usage 变化
  $: if (usage?.data?.limits) {
    limits = usage.data.limits;
    lastUpdate = new Date();
  }

  async function refresh() {
    const result = await invoke("get_current_usage");
    usage = result;
    error = await invoke("get_current_error");
    lastUpdate = new Date();
  }

  function formatNumber(num: number): string {
    if (num >= 1_000_000) return `${(num / 1_000_000).toFixed(1)}M`;
    if (num >= 1_000) return `${(num / 1_000).toFixed(1)}K`;
    return num.toString();
  }

  function getLimitTitle(limit: any): string {
    switch (limit.limit_type) {
      case "TIME_LIMIT":
        return "时间限额";
      case "TOKENS_LIMIT":
        return "Token 限额";
      default:
        return limit.limit_type;
    }
  }

  function getResetTimeText(limit: any): string {
    if (limit.next_reset_time) {
      // 将时间戳转换为可读格式
      const date = new Date(limit.next_reset_time);
      return `重置时间: ${formatDate(date)}`;
    }
    if (limit.limit_type === "TIME_LIMIT") {
      switch (limit.unit) {
        case 1:
          return "每小时重置";
        case 5:
          return "每5小时重置";
        case 24:
          return "每天重置";
        default:
          return `每${limit.unit}小时重置`;
      }
    }
    return "重置时间: 未知";
  }

  function formatDate(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    const hours = String(date.getHours()).padStart(2, "0");
    const minutes = String(date.getMinutes()).padStart(2, "0");
    return `${year}-${month}-${day} ${hours}:${minutes}`;
  }

  function formatLastUpdate(): string {
    if (!lastUpdate) return "未更新";
    return formatDate(lastUpdate);
  }

  function getProgressColor(percentage: number): string {
    if (percentage >= 90) return "#ff4d4f";
    if (percentage >= 70) return "#faad14";
    return "#1890ff";
  }
</script>

<div class="usage-display">
  <div class="header">
    <button class="config-btn" on:click={onConfig}>配置</button>
  </div>

  {#if error}
    <div class="error-banner">
      <span class="icon">⚠</span>
      <span>{error}</span>
    </div>
  {/if}

  {#if limits.length > 0}
    <div class="card-container">
      {#each limits as limit (limit.limit_type)}
        <div class="card">
          <div class="card-header">
            <h3 class="title">{getLimitTitle(limit)}</h3>
            <span class="info-icon">ℹ</span>
          </div>

          <div class="progress-section">
            <span class="percentage">{limit.percentage.toFixed(0)}%</span>
            <span class="text">已使用</span>
          </div>

          <div class="progress-bar">
            <div
              class="progress-fill"
              style="width: {limit.percentage}%; background-color: {getProgressColor(limit.percentage)}"
            ></div>
          </div>

          <p class="reset-time">{getResetTimeText(limit)}</p>

          {#if limit.usage_details && limit.usage_details.length > 0}
            <div class="usage-details">
              <h4>使用详情</h4>
              {#each limit.usage_details as detail}
                <div class="detail-item">
                  <span class="model-name">{detail.model_code}</span>
                  <span class="model-usage">{detail.usage} 次</span>
                </div>
              {/each}
            </div>
          {/if}

          <div class="extra-info">
            {formatNumber(limit.current_value)} / {formatNumber(limit.usage)}
          </div>
        </div>
      {/each}
    </div>

    <div class="footer">
      <span>最近更新时间: {formatLastUpdate()}</span>
      <button class="refresh-btn" on:click={refresh}>刷新</button>
    </div>
  {:else}
    <div class="loading">
      <p>正在加载用量数据...</p>
      <button on:click={refresh}>手动刷新</button>
    </div>
  {/if}
</div>

<style>
  .usage-display {
    min-width: 500px;
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  }

  .header {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 16px;
  }

  .config-btn {
    padding: 6px 16px;
    background: #fff;
    border: 1px solid #d9d9d9;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
  }

  .config-btn:hover {
    border-color: #1890ff;
    color: #1890ff;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    background: #fff2f0;
    border: 1px solid #ffccc7;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 14px;
    color: #ff4d4f;
  }

  .error-banner .icon {
    font-size: 16px;
  }

  .card-container {
    display: flex;
    gap: 16px;
    margin-bottom: 20px;
  }

  .card {
    flex: 1;
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
    padding: 20px;
    display: flex;
    flex-direction: column;
  }

  .card-header {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }

  .title {
    font-size: 16px;
    font-weight: 500;
    color: #262626;
    margin: 0;
  }

  .info-icon {
    margin-left: 8px;
    font-size: 14px;
    color: #1890ff;
    cursor: help;
  }

  .progress-section {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 8px;
  }

  .percentage {
    font-size: 32px;
    font-weight: bold;
    color: #262626;
  }

  .text {
    font-size: 14px;
    color: #8c8c8c;
  }

  .progress-bar {
    height: 8px;
    border-radius: 4px;
    background-color: #e8f4ff;
    overflow: hidden;
    margin-bottom: 12px;
  }

  .progress-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.3s ease, background-color 0.3s ease;
  }

  .reset-time {
    font-size: 12px;
    color: #8c8c8c;
    margin: 0 0 12px 0;
  }

  .usage-details {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid #f0f0f0;
  }

  .usage-details h4 {
    font-size: 13px;
    font-weight: 500;
    color: #595959;
    margin: 0 0 8px 0;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    padding: 4px 0;
  }

  .model-name {
    color: #8c8c8c;
  }

  .model-usage {
    color: #262626;
    font-weight: 500;
  }

  .extra-info {
    font-size: 14px;
    color: #8c8c8c;
    text-align: right;
    margin-top: auto;
    padding-top: 8px;
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 12px;
    color: #595959;
    padding: 0 4px;
  }

  .refresh-btn {
    border: none;
    background: none;
    color: #1890ff;
    cursor: pointer;
    font-size: 12px;
    padding: 4px 8px;
  }

  .refresh-btn:hover {
    text-decoration: underline;
  }

  .loading {
    text-align: center;
    padding: 60px 20px;
    color: #8c8c8c;
  }

  .loading p {
    margin-bottom: 16px;
  }

  .loading button {
    padding: 8px 20px;
    background: #1890ff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .loading button:hover {
    background: #40a9ff;
  }
</style>
