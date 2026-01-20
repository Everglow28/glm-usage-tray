<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let config: any;
  export let onSave: (config: any) => void;
  export let onClose: () => void;

  let token = "";
  let organization = "";
  let project = "";
  let refreshInterval = 60;
  let testing = false;
  let testResult: { type: "success" | "error"; message: string } | null = null;

  // 当 config prop 更新时，同步到本地变量
  $: if (config) {
    token = config.token || "";
    organization = config.organization || "";
    project = config.project || "";
    refreshInterval = config.refresh_interval || 60;
  }

  async function testConnection() {
    if (!token || !organization || !project) {
      testResult = { type: "error", message: "请填写完整的配置信息" };
      return;
    }

    testing = true;
    testResult = null;

    try {
      const result = await invoke("test_connection", {
        config: {
          token,
          organization,
          project,
          refresh_interval: refreshInterval,
        },
      });

      // 从新的 API 结构中提取数据
      const data = result as any;
      console.log("Test connection result:", data);

      if (data?.data?.limits && data.data.limits.length > 0) {
        const tokenLimit = data.data.limits.find((l: any) => l.limit_type === "TOKENS_LIMIT");
        console.log("Found token limit:", tokenLimit);
        if (tokenLimit) {
          testResult = {
            type: "success",
            message: `连接成功！Token: ${formatNumber(tokenLimit.current_value)} / ${formatNumber(tokenLimit.usage)} (${tokenLimit.percentage.toFixed(0)}%)`,
          };
        } else {
          testResult = { type: "success", message: "连接成功！但未找到 Token 限额数据" };
        }
      } else {
        testResult = { type: "error", message: "API 返回数据格式异常" };
      }
    } catch (e: any) {
      console.error("Test connection error:", e);
      testResult = { type: "error", message: String(e) };
    } finally {
      testing = false;
    }
  }

  async function save() {
    if (!token || !organization || !project) {
      testResult = { type: "error", message: "请填写完整的配置信息" };
      return;
    }

    try {
      await invoke("save_config_cmd", {
        config: {
          token,
          organization,
          project,
          refresh_interval: refreshInterval,
        },
      });
      onSave({ token, organization, project, refresh_interval: refreshInterval });
      testResult = { type: "success", message: "配置已保存" };
      setTimeout(() => onClose(), 500);
    } catch (e: any) {
      testResult = { type: "error", message: String(e) };
    }
  }

  function formatNumber(n: number): string {
    if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
    if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
    return n.toString();
  }
</script>

<div class="config-panel">
  <div class="panel-header">
    <h1 class="panel-title">GLM 用量配置</h1>
    <button class="close-btn" on:click={onClose}>✕</button>
  </div>

  <div class="form-section">
    <div class="form-group">
      <label for="token">Authorization Token</label>
      <input
        id="token"
        type="password"
        bind:value={token}
        placeholder="从浏览器 Cookie 复制"
      />
      <span class="help-text">从浏览器 Cookie 中的 bigmodel_token_production 复制</span>
    </div>

    <div class="form-group">
      <label for="org">Organization ID</label>
      <input
        id="org"
        bind:value={organization}
        placeholder="例如: org-xxxxxxxx"
      />
      <span class="help-text">从请求头 bigmodel-organization 复制</span>
    </div>

    <div class="form-group">
      <label for="project">Project ID</label>
      <input
        id="project"
        bind:value={project}
        placeholder="例如: proj-xxxxxxxx"
      />
      <span class="help-text">从请求头 bigmodel-project 复制</span>
    </div>

    <div class="form-group">
      <label for="interval">刷新间隔</label>
      <select id="interval" bind:value={refreshInterval}>
        <option value={30}>30 秒</option>
        <option value={60}>60 秒 (推荐)</option>
        <option value={120}>120 秒</option>
        <option value={300}>300 秒</option>
      </select>
    </div>
  </div>

  {#if testResult}
    <div class="alert {testResult.type}">
      <span class="alert-icon">{testResult.type === "success" ? "✓" : "⚠"}</span>
      <span class="alert-message">{testResult.message}</span>
    </div>
  {/if}

  <div class="actions">
    <button class="btn-primary" on:click={save}>
      保存配置
    </button>
    <button class="btn-secondary" on:click={testConnection} disabled={testing}>
      {testing ? "测试中..." : "测试连接"}
    </button>
  </div>
</div>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap');

  .config-panel {
    max-width: 520px;
    margin: 0 auto;
    font-family: 'Outfit', sans-serif;
    background: #ffffff;
    border: 2px solid #e5e5e5;
    border-radius: 16px;
    padding: 32px;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 28px;
    padding-bottom: 16px;
    border-bottom: 2px solid #e5e5e5;
  }

  .panel-title {
    font-size: 24px;
    font-weight: 700;
    color: #0a0a0a;
    margin: 0;
  }

  .close-btn {
    width: 32px;
    height: 32px;
    border: 2px solid #e5e5e5;
    border-radius: 8px;
    background: #ffffff;
    color: #0a0a0a;
    font-size: 18px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background: #f5f5f5;
    border-color: #0a0a0a;
  }

  .form-section {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-size: 14px;
    font-weight: 600;
    color: #0a0a0a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  input,
  select {
    width: 100%;
    padding: 12px 16px;
    font-size: 14px;
    font-family: 'JetBrains Mono', monospace;
    background: #ffffff;
    border: 2px solid #e5e5e5;
    border-radius: 8px;
    color: #0a0a0a;
    transition: all 0.15s ease;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #0a0a0a;
  }

  input::placeholder {
    color: #a3a3a3;
  }

  .help-text {
    font-size: 12px;
    color: #737373;
  }

  .alert {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    border-radius: 12px;
    margin-top: 8px;
    font-size: 14px;
    font-weight: 500;
  }

  .alert.success {
    background: #f0fdf4;
    border: 2px solid #86efac;
    color: #166534;
  }

  .alert.error {
    background: #fef2f2;
    border: 2px solid #fecaca;
    color: #991b1b;
  }

  .alert-icon {
    font-size: 20px;
  }

  .alert-message {
    flex: 1;
  }

  .actions {
    display: flex;
    gap: 12px;
    margin-top: 28px;
  }

  .actions button {
    flex: 1;
    padding: 14px 24px;
    font-size: 15px;
    font-weight: 600;
    font-family: 'Outfit', sans-serif;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.15s ease;
    border: none;
  }

  .btn-primary {
    background: #0a0a0a;
    color: #ffffff;
  }

  .btn-primary:hover:not(:disabled) {
    background: #262626;
  }

  .btn-secondary {
    background: #ffffff;
    color: #0a0a0a;
    border: 2px solid #e5e5e5;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #f5f5f5;
    border-color: #0a0a0a;
  }

  .actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
