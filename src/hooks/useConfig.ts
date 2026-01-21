import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { AppConfig } from '../types/api';

export function useConfig() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [showConfig, setShowConfig] = useState(true);

  useEffect(() => {
    invoke<AppConfig>("get_config").then(setConfig);

    const unlistenPromise = listen("tray-click", () => {
      setShowConfig(prev => !prev);
    });

    return () => {
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  const handleSave = useCallback((newConfig: AppConfig) => {
    setConfig(newConfig);
  }, []);

  return { config, showConfig, setShowConfig, handleSave };
}
