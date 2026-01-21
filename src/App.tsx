import { useEffect, useState, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { AppConfig, UsageData } from './types/api';
import ConfigPanel from './components/ConfigPanel';
import UsageDisplay from './components/UsageDisplay';
import './app.css';
export default function App() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [usage, setUsage] = useState<UsageData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showConfig, setShowConfig] = useState(true);
  const unlistensRef = useRef<(() => void)[]>([]);

  // 手动刷新函数
  const handleRefresh = useCallback(async () => {
    try {
      const result = await invoke<UsageData>('manual_refresh');
      if (result) {
        setUsage(result);
        setError(null);
      }
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    let mounted = true;
    const unlistens: (() => void)[] = [];

    // 先设置事件监听器
    const setupListeners = async () => {
      const u1 = await listen<UsageData>('usage-update', (event) => {
        if (mounted) {
          setUsage(event.payload);
          setError(null);
        }
      });
      unlistens.push(u1);

      const u2 = await listen<string>('usage-error', (event) => {
        if (mounted) {
          setError(event.payload);
        }
      });
      unlistens.push(u2);

      const u3 = await listen('tray-click', () => {
        if (mounted) {
          setShowConfig((prev) => !prev);
        }
      });
      unlistens.push(u3);

      // 所有监听器设置完成后，保存 unlisten 函数
      if (mounted) {
        unlistensRef.current = unlistens;
      }

      if (mounted) {
        // 所有监听器设置完成后，保存 unlisten 函数
        unlistensRef.current = unlistens;
        // 加载配置
        invoke<AppConfig>('get_config').then((cfg) => {
          if (mounted) {
            setConfig(cfg);
            // 如果有配置且有效，默认显示用量页面
            if (cfg && cfg.token && cfg.organization && cfg.project) {
              setShowConfig(false);
            }
          }
        });

        // 获取当前状态
        invoke<UsageData | null>('get_current_usage').then((data) => {
          if (mounted && data) {
            setUsage(data);
          }
        });

        invoke<string | null>('get_current_error').then((err) => {
          if (mounted && err) {
            setError(err);
          }
        });
      }
    };

    setupListeners();

    return () => {
      mounted = false;
      // 清理所有监听器
      unlistensRef.current.forEach((unlisten) => unlisten());
    };
  }, []);

  const handleSave = (newConfig: AppConfig) => {
    setConfig(newConfig);
  };

  return (
    <div className='container'>
      {showConfig || !config ? (
        <ConfigPanel config={config} onSave={handleSave} onClose={() => setShowConfig(false)} />
      ) : (
        <UsageDisplay usage={usage} error={error} onConfig={() => setShowConfig(true)} onRefresh={handleRefresh} />
      )}
    </div>
  );
}
