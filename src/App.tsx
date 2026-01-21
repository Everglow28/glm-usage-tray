import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { AppConfig, UsageData } from './types/api';
import ConfigPanel from './components/ConfigPanel';
import UsageDisplay from './components/UsageDisplay';
import './App.css';

export default function App() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [usage, setUsage] = useState<UsageData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showConfig, setShowConfig] = useState(true);

  useEffect(() => {
    let mounted = true;

    // 加载配置
    invoke<AppConfig>("get_config").then(cfg => {
      if (mounted) {
        setConfig(cfg);
        // 如果有配置且有效，默认显示用量页面
        if (cfg && cfg.token && cfg.organization && cfg.project) {
          setShowConfig(false);
        }
      }
    });

    // 先设置事件监听器（确保不丢失后续事件）
    listen<UsageData>("usage-update", (event) => {
      if (mounted) {
        setUsage(event.payload);
        setError(null);
      }
    });

    listen<string>("usage-error", (event) => {
      if (mounted) {
        setError(event.payload);
      }
    });

    listen("tray-click", () => {
      if (mounted) {
        setShowConfig(prev => !prev);
      }
    });

    // 然后获取当前状态（在监听器设置之后）
    invoke<UsageData | null>("get_current_usage").then(data => {
      if (mounted && data) {
        setUsage(data);
      }
    });

    invoke<string | null>("get_current_error").then(err => {
      if (mounted && err) {
        setError(err);
      }
    });

    return () => {
      mounted = false;
    };
  }, []);

  const handleSave = (newConfig: AppConfig) => {
    setConfig(newConfig);
  };

  return (
    <div className="container">
      {showConfig || !config ? (
        <ConfigPanel
          config={config}
          onSave={handleSave}
          onClose={() => setShowConfig(false)}
        />
      ) : (
        <UsageDisplay
          usage={usage}
          error={error}
          onConfig={() => setShowConfig(true)}
        />
      )}
    </div>
  );
}
