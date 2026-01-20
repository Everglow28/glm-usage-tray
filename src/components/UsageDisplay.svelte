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
      const date = new Date(limit.next_reset_time);
      return `重置: ${formatDate(date)}`;
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
    return "重置: 未知";
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
    if (percentage >= 90) return "#dc2626"; // 红色 - 危险
    if (percentage >= 70) return "#f59e0b"; // 橙色 - 警告
    return "#2563eb"; // 蓝色 - 正常
  }
</script>

<div class="usage-display">
  <!-- 顶部栏 -->
  <div class="top-bar">
    <div class="title-section">
      <h1 class="app-title">GLM 用量监控</h1>
      <div class="status-dot"></div>
    </div>
    <button class="config-btn" on:click={onConfig}>
      <span class="icon">⚙</span> 配置
    </button>
  </div>

  {#if error}
    <div class="error-banner">
      <span class="error-icon">⚠</span>
      <span class="error-text">{error}</span>
    </div>
  {/if}

  {#if limits.length > 0}
    <div class="cards-grid">
      {#each limits as limit (limit.limit_type)}
        <div class="limit-card">
          <!-- 卡片标题 -->
          <div class="card-header">
            <h2 class="card-title">{getLimitTitle(limit)}</h2>
            <span class="card-type">{limit.limit_type}</span>
          </div>

          <!-- 百分比显示 -->
          <div class="percentage-display">
            <span class="percentage-value">{limit.percentage.toFixed(0)}%</span>
            <span class="percentage-label">已使用</span>
          </div>

          <!-- 进度条 -->
          <div class="progress-container">
            <div class="progress-track">
              <div
                class="progress-fill"
                style="width: {limit.percentage}%; background-color: {getProgressColor(limit.percentage)}"
              ></div>
            </div>
          </div>

          <!-- 数值详情 -->
          <div class="values-row">
            <div class="value-item">
              <span class="value-label">已用</span>
              <span class="value-number">{formatNumber(limit.current_value)}</span>
            </div>
            <div class="value-divider">/</div>
            <div class="value-item">
              <span class="value-label">总额</span>
              <span class="value-number">{formatNumber(limit.usage)}</span>
            </div>
            <div class="value-item">
              <span class="value-label">剩余</span>
              <span class="value-number">{formatNumber(limit.remaining)}</span>
            </div>
          </div>

          <!-- 重置时间 -->
          <div class="reset-info">
            <span class="reset-icon">↻</span>
            <span class="reset-text">{getResetTimeText(limit)}</span>
          </div>

          <!-- 使用详情 -->
          {#if limit.usage_details && limit.usage_details.length > 0}
            <div class="details-section">
              <h3 class="details-title">模型使用详情</h3>
              <div class="details-list">
                {#each limit.usage_details as detail}
                  <div class="detail-row">
                    <span class="detail-name">{detail.model_code}</span>
                    <span class="detail-value">{detail.usage}</span>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>

    <!-- 底部栏 -->
    <div class="bottom-bar">
      <div class="update-info">
        <span class="update-icon">◷</span>
        <span class="update-text">{formatLastUpdate()}</span>
      </div>
      <button class="refresh-btn" on:click={refresh}>
        <span class="refresh-icon">⟳</span> 刷新
      </button>
    </div>
  {:else}
    <div class="loading-state">
      <div class="loading-spinner"></div>
      <p class="loading-text">正在加载用量数据...</p>
      <button class="loading-btn" on:click={refresh}>手动刷新</button>
    </div>
  {/if}
</div>

<style>
  /* 导入字体 - 使用几何无衬线 + 等宽字体 */
  @import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&display=swap');

  .usage-display {
    min-width: 640px;
    max-width: 900px;
    margin: 0 auto;
    padding: 24px;
    font-family: 'Outfit', -apple-system, BlinkMacSystemFont, sans-serif;
    background: linear-gradient(135deg, #fafafa 0%, #f5f5f5 100%);
    min-height: 100vh;
  }

  /* 网格背景效果 */
  .usage-display::before {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image:
      linear-gradient(rgba(0, 0, 0, 0.02) 1px, transparent 1px),
      linear-gradient(90deg, rgba(0, 0, 0, 0.02) 1px, transparent 1px);
    background-size: 20px 20px;
    pointer-events: none;
    z-index: -1;
  }

  /* 顶部栏 */
  .top-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 2px solid #e5e5e5;
  }

  .title-section {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .app-title {
    font-size: 24px;
    font-weight: 700;
    color: #0a0a0a;
    margin: 0;
    letter-spacing: -0.5px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    background: #22c55e;
    border-radius: 50%;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .config-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 18px;
    background: #ffffff;
    border: 2px solid #d4d4d4;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 600;
    color: #0a0a0a;
    font-family: 'Outfit', sans-serif;
    transition: all 0.15s ease;
  }

  .config-btn:hover {
    background: #f5f5f5;
    border-color: #0a0a0a;
  }

  .config-btn:active {
    transform: translateY(1px);
  }

  .config-btn .icon {
    font-size: 16px;
  }

  /* 错误横幅 */
  .error-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    background: #fef2f2;
    border: 2px solid #fecaca;
    border-radius: 12px;
    margin-bottom: 20px;
  }

  .error-icon {
    font-size: 20px;
  }

  .error-text {
    font-size: 14px;
    font-weight: 500;
    color: #991b1b;
  }

  /* 卡片网格 */
  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 20px;
    margin-bottom: 20px;
  }

  /* 卡片 */
  .limit-card {
    background: #ffffff;
    border: 2px solid #e5e5e5;
    border-radius: 16px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    transition: all 0.2s ease;
  }

  .limit-card:hover {
    border-color: #d4d4d4;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }

  /* 卡片头部 */
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .card-title {
    font-size: 18px;
    font-weight: 700;
    color: #0a0a0a;
    margin: 0;
  }

  .card-type {
    font-family: 'JetBrains Mono', monospace;
    font-size: 11px;
    font-weight: 600;
    color: #737373;
    background: #f5f5f5;
    padding: 4px 10px;
    border-radius: 6px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  /* 百分比显示 */
  .percentage-display {
    display: flex;
    align-items: baseline;
    gap: 10px;
  }

  .percentage-value {
    font-family: 'JetBrains Mono', monospace;
    font-size: 48px;
    font-weight: 700;
    color: #0a0a0a;
    line-height: 1;
  }

  .percentage-label {
    font-size: 14px;
    font-weight: 500;
    color: #737373;
  }

  /* 进度条 */
  .progress-container {
    width: 100%;
  }

  .progress-track {
    height: 12px;
    background: #f5f5f5;
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid #e5e5e5;
  }

  .progress-fill {
    height: 100%;
    border-radius: 7px;
    transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1), background-color 0.3s ease;
  }

  /* 数值行 */
  .values-row {
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 16px 0;
    border-top: 1px solid #e5e5e5;
    border-bottom: 1px solid #e5e5e5;
  }

  .value-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .value-label {
    font-size: 12px;
    font-weight: 500;
    color: #737373;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .value-number {
    font-family: 'JetBrains Mono', monospace;
    font-size: 20px;
    font-weight: 700;
    color: #0a0a0a;
  }

  .value-divider {
    font-size: 20px;
    font-weight: 400;
    color: #d4d4d4;
  }

  /* 重置信息 */
  .reset-info {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 500;
    color: #525252;
  }

  .reset-icon {
    font-size: 14px;
    color: #737373;
  }

  /* 使用详情 */
  .details-section {
    padding-top: 16px;
    border-top: 1px dashed #e5e5e5;
  }

  .details-title {
    font-size: 13px;
    font-weight: 600;
    color: #0a0a0a;
    margin: 0 0 12px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .details-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: #fafafa;
    border-radius: 8px;
    font-size: 13px;
  }

  .detail-name {
    font-weight: 500;
    color: #525252;
  }

  .detail-value {
    font-family: 'JetBrains Mono', monospace;
    font-weight: 600;
    color: #0a0a0a;
  }

  /* 底部栏 */
  .bottom-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-top: 2px solid #e5e5e5;
  }

  .update-info {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 500;
    color: #525252;
  }

  .update-icon {
    font-size: 14px;
    color: #737373;
  }

  .refresh-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 18px;
    background: #0a0a0a;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
    font-family: 'Outfit', sans-serif;
    transition: all 0.15s ease;
  }

  .refresh-btn:hover {
    background: #262626;
    transform: translateY(-1px);
  }

  .refresh-btn:active {
    transform: translateY(0);
  }

  .refresh-icon {
    font-size: 16px;
  }

  /* 加载状态 */
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 20px;
    gap: 20px;
  }

  .loading-spinner {
    width: 48px;
    height: 48px;
    border: 4px solid #e5e5e5;
    border-top-color: #0a0a0a;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .loading-text {
    font-size: 16px;
    font-weight: 500;
    color: #525252;
    margin: 0;
  }

  .loading-btn {
    padding: 12px 24px;
    background: #0a0a0a;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
    font-family: 'Outfit', sans-serif;
    transition: all 0.15s ease;
  }

  .loading-btn:hover {
    background: #262626;
  }
</style>
