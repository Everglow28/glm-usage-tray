<script lang="ts">
  import { onMount } from "svelte";
  import ConfigPanel from "./components/ConfigPanel.svelte";
  import UsageDisplay from "./components/UsageDisplay.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  let config = $state<any>(null);
  let usage = $state<any>(null);
  let error = $state<string | null>(null);
  let showConfig = $state(true);

  onMount(async () => {
    // 加载配置
    config = await invoke("get_config");

    // 监听用量更新
    const unlistenUsage = await listen("usage-update", (event: any) => {
      usage = event.payload;
      error = null;
    });

    // 监听错误
    const unlistenError = await listen("usage-error", (event: any) => {
      error = event.payload;
    });

    // 监听托盘点击
    const unlistenTray = await listen("tray-click", () => {
      showConfig = !showConfig;
    });

    // 获取当前状态
    usage = await invoke("get_current_usage");
    error = await invoke("get_current_error");

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
  .container {
    width: 100%;
    height: 100%;
    padding: 20px;
  }
</style>
