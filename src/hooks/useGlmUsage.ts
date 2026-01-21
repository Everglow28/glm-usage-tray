import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UsageData, UsageError } from '../types/api';

interface UsageResponse {
  success: boolean;
  data?: {
    limits: any[];
  };
  code?: string;
  message?: string;
}

export function useGlmUsage() {
  const [usage, setUsage] = useState<UsageData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const unlistenRef = useRef<(() => Promise<void>) | null>(null);

  // 手动刷新
  const refresh = useCallback(async () => {
    try {
      const result = await invoke<UsageResponse>("manual_refresh");
      if (result) {
        if (result.success && result.data) {
          setUsage(result as UsageData);
          setError(null);
        } else {
          setError((result as UsageError).message || "未知错误");
        }
      }
    } catch (e) {
      setError(String(e));
    }
  }, []);

  // 初始化和事件监听
  useEffect(() => {
    let mounted = true;

    // 获取初始数据
    invoke<UsageData | null>("get_current_usage").then(data => {
      if (mounted && data) {
        setUsage(data);
      }
    }).catch(e => {
      if (mounted) {
        setError(String(e));
      }
    });

    invoke<string | null>("get_current_error").then(err => {
      if (mounted && err) {
        setError(err);
      }
    });

    // 监听自动刷新事件
    listen<UsageData>("usage-update", (event) => {
      if (mounted) {
        setUsage(event.payload);
        setError(null);
      }
    }).then(unlisten => {
      if (mounted) {
        unlistenRef.current = unlisten;
      }
    });

    // 监听错误事件
    listen<string>("usage-error", (event) => {
      if (mounted) {
        setError(event.payload);
      }
    });

    // 清理函数（防止 HMR 重复监听）
    return () => {
      mounted = false;
      unlistenRef.current?.();
    };
  }, []);

  return { usage, error, refresh };
}
