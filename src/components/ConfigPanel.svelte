<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let config: any;
  export let onSave: (config: any) => void;
  export let onClose: () => void;

  let token = $state(config?.token || "");
  let organization = $state(config?.organization || "");
  let project = $state(config?.project || "");
  let refreshInterval = $state(config?.refresh_interval || 60);
  let testing = $state(false);
  let testResult = $state<{ type: "success" | "error"; message: string } | null>(null);

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
      testResult = {
        type: "success",
        message: `连接成功！已用: ${formatNumber(result.used_tokens)} / ${formatNumber(result.total_tokens)}`,
      };
    } catch (e: any) {
      testResult = { type: "error", message: e as string };
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
      testResult = { type: "error", message: e as string };
    }
  }

  function formatNumber(n: number): string {
    if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
    if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
    return n.toString();
  }
</script>

<div class="config-panel">
  <h1>GLM Usage 配置</h1>

  <div class="form-group">
    <label for="token">Authorization Token</label>
    <input
      id="token"
      type="password"
      bind:value={token}
      placeholder="从浏览器 Cookie 中的 bigmodel_token_production 复制"
    />
  </div>

  <div class="form-group">
    <label for="org">Organization ID</label>
    <input
      id="org"
      bind:value={organization}
      placeholder="从请求头 bigmodel-organization 复制"
    />
  </div>

  <div class="form-group">
    <label for="project">Project ID</label>
    <input
      id="project"
      bind:value={project}
      placeholder="从请求头 bigmodel-project 复制"
    />
  </div>

  <div class="form-group">
    <label for="interval">刷新间隔 (秒)</label>
    <select id="interval" bind:value={refreshInterval}>
      <option value={30}>30</option>
      <option value={60}>60</option>
      <option value={120}>120</option>
      <option value={300}>300</option>
    </select>
  </div>

  {#if testResult}
    <div class="alert {testResult.type}">
      {testResult.message}
    </div>
  {/if}

  <div class="actions">
    <button class="primary" on:click={save}>保存配置</button>
    <button on:click={testConnection} disabled={testing}>
      {testing ? "测试中..." : "测试连接"}
    </button>
  </div>
</div>

<style>
  .config-panel {
    max-width: 450px;
    margin: 0 auto;
  }

  h1 {
    font-size: 1.5rem;
    margin-bottom: 20px;
    text-align: center;
  }

  .form-group {
    margin-bottom: 15px;
  }

  label {
    display: block;
    margin-bottom: 5px;
    font-weight: 500;
  }

  input,
  select {
    width: 100%;
  }

  input {
    font-family: monospace;
    font-size: 0.9rem;
  }

  .alert {
    padding: 10px;
    border-radius: 4px;
    margin: 15px 0;
    font-size: 0.9rem;
  }

  .alert.success {
    background: rgba(81, 207, 102, 0.2);
    border: 1px solid #51cf66;
  }

  .alert.error {
    background: rgba(255, 107, 107, 0.2);
    border: 1px solid #ff6b6b;
  }

  .actions {
    display: flex;
    gap: 10px;
    margin-top: 20px;
  }

  .actions button {
    flex: 1;
  }
</style>
