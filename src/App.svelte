<script lang="ts">
  import { onMount } from "svelte";
  import ConfigPanel from "./components/ConfigPanel.svelte";
  import UsageDisplay from "./components/UsageDisplay.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  let config: any = null;
  let usage: any = null;
  let error: string | null = null;
  let showConfig = true;

  onMount(async () => {
    // 加载配置
    config = await invoke("get_config");

    // 先设置事件监听器（确保不丢失后续事件）
    const unlistenUsage = await listen("usage-update", (event: any) => {
      usage = event.payload;
      error = null;
    });

    const unlistenError = await listen("usage-error", (event: any) => {
      error = event.payload;
    });

    const unlistenTray = await listen("tray-click", () => {
      showConfig = !showConfig;
    });

    // 然后获取当前状态（在监听器设置之后）
    usage = await invoke("get_current_usage");
    error = await invoke("get_current_error");

    // 如果有配置且有效，默认显示用量页面
    if (config && config.token && config.organization && config.project) {
      showConfig = false;
    }

    return () => {
      unlistenUsage();
      unlistenError();
      unlistenTray();
    };
  });

  function handleSave(newConfig: any) {
    config = newConfig;
  }
</script>

<div class="container">
  {#if showConfig || !config}
    <ConfigPanel {config} onSave={handleSave} onClose={() => showConfig = false} />
  {:else}
    <UsageDisplay {usage} {error} onConfig={() => showConfig = true} />
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: #f5f5f5;
  }

  .container {
    width: 100%;
    height: 100vh;
    overflow-y: auto;
  }
</style>
