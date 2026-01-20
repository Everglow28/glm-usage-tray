<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let usage: any;
  export let error: string | null;
  export let onConfig: () => void;

  let lastUpdate: Date | null = null;

  async function refresh() {
    const result = await invoke("get_current_usage");
    usage = result;
    error = await invoke("get_current_error");
    lastUpdate = new Date();
  }

  function formatTokens(tokens: number): string {
    if (tokens >= 1_000_000) return `${(tokens / 1_000_000).toFixed(1)}M`;
    if (tokens >= 1_000) return `${(tokens / 1_000).toFixed(1)}K`;
    return tokens.toString();
  }

  function getStatusColor(percentage: number): string {
    if (percentage >= 90) return "#ff6b6b";
    if (percentage >= 70) return "#ffd43b";
    return "#51cf66";
  }
</script>

<div class="usage-display">
  <header>
    <h1>GLM Coding Plan 用量</h1>
    <button on:click={onConfig}>配置</button>
  </header>

  {#if error}
    <div class="error-banner">
      <span class="icon">⚠</span>
      <span>{error}</span>
    </div>
  {/if}

  {#if usage}
    <div class="usage-content">
      <div class="progress-container">
        <div class="progress-bar">
          <div
            class="progress-fill"
            style="width: {usage.usage_percentage}%; background: {getStatusColor(usage.usage_percentage)}"
          ></div>
        </div>
        <div class="percentage">{usage.usage_percentage.toFixed(1)}%</div>
      </div>

      <div class="stats">
        <div class="stat">
          <span class="label">已使用</span>
          <span class="value">{formatTokens(usage.used_tokens)}</span>
        </div>
        <div class="stat">
          <span class="label">总计</span>
          <span class="value">{formatTokens(usage.total_tokens)}</span>
        </div>
        <div class="stat">
          <span class="label">剩余</span>
          <span class="value">{formatTokens(usage.remaining_tokens)}</span>
        </div>
      </div>

      <button class="refresh-btn" on:click={refresh}>
        刷新
      </button>
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
    max-width: 400px;
    margin: 0 auto;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  header h1 {
    font-size: 1.2rem;
    margin: 0;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    background: rgba(255, 107, 107, 0.2);
    border: 1px solid #ff6b6b;
    border-radius: 4px;
    margin-bottom: 15px;
    font-size: 0.9rem;
  }

  .icon {
    font-size: 1.2rem;
  }

  .progress-container {
    margin-bottom: 25px;
  }

  .progress-bar {
    width: 100%;
    height: 24px;
    background: #2a2a2a;
    border-radius: 4px;
    overflow: hidden;
    border: 1px solid #444;
  }

  .progress-fill {
    height: 100%;
    transition: width 0.3s ease, background 0.3s ease;
  }

  .percentage {
    text-align: center;
    font-size: 1.5rem;
    font-weight: bold;
    margin-top: 10px;
  }

  .stats {
    display: flex;
    justify-content: space-around;
    margin-bottom: 20px;
  }

  .stat {
    text-align: center;
  }

  .stat .label {
    display: block;
    font-size: 0.8rem;
    opacity: 0.7;
    margin-bottom: 5px;
  }

  .stat .value {
    display: block;
    font-size: 1.2rem;
    font-weight: bold;
  }

  .refresh-btn {
    width: 100%;
  }

  .loading {
    text-align: center;
    padding: 40px 0;
  }

  .loading p {
    opacity: 0.7;
    margin-bottom: 15px;
  }
</style>
