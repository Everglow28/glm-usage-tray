import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import clsx from 'clsx';
import type { AppConfig } from '../types/api';
import styles from './ConfigPanel.module.css';

interface ConfigPanelProps {
  config: AppConfig | null;
  onSave: (config: AppConfig) => void;
  onClose: () => void;
}

interface TestResult {
  type: 'success' | 'error';
  message: string;
}

export default function ConfigPanel({ config, onSave, onClose }: ConfigPanelProps) {
  const [token, setToken] = useState('');
  const [organization, setOrganization] = useState('');
  const [project, setProject] = useState('');
  const [refreshInterval, setRefreshInterval] = useState(60);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<TestResult | null>(null);

  // 当 config prop 更新时，同步到本地变量
  useEffect(() => {
    if (config) {
      setToken(config.token || '');
      setOrganization(config.organization || '');
      setProject(config.project || '');
      setRefreshInterval(config.refresh_interval || 60);
    }
  }, [config]);

  const testConnection = async () => {
    if (!token || !organization || !project) {
      setTestResult({ type: 'error', message: '请填写完整的配置信息' });
      return;
    }

    setTesting(true);
    setTestResult(null);

    try {
      const result = await invoke('test_connection', {
        config: {
          token,
          organization,
          project,
          refresh_interval: refreshInterval,
        },
      });

      const data = result as any;
      console.log('=== 测试连接调试 ===');
      console.log('完整响应:', JSON.stringify(data, null, 2));
      console.log('limits 数组:', data?.data?.limits);
      console.log('limits 长度:', data?.data?.limits?.length);
      if (data?.data?.limits && data.data.limits.length > 0) {
        console.log('第一个 limit:', JSON.stringify(data.data.limits[0], null, 2));
        console.log('第一个 limit.type:', data.data.limits[0].type);
        console.log('字段枚举:', Object.keys(data.data.limits[0]));
      }

      if (data?.data?.limits && data.data.limits.length > 0) {
        const tokenLimit = data.data.limits.find((l: any) => l.type === 'TOKENS_LIMIT');
        console.log('查找结果 tokenLimit:', tokenLimit);
        if (tokenLimit) {
          setTestResult({
            type: 'success',
            message: `连接成功！Token: ${formatNumber(tokenLimit.currentValue)} / ${formatNumber(tokenLimit.usage)} (${tokenLimit.percentage.toFixed(0)}%)`,
          });
        } else {
          setTestResult({ type: 'success', message: '连接成功！但未找到 Token 限额数据' });
        }
      } else {
        setTestResult({ type: 'error', message: 'API 返回数据格式异常' });
      }
    } catch (e: any) {
      console.error('Test connection error:', e);
      setTestResult({ type: 'error', message: String(e) });
    } finally {
      setTesting(false);
    }
  };

  const save = async () => {
    if (!token || !organization || !project) {
      setTestResult({ type: 'error', message: '请填写完整的配置信息' });
      return;
    }

    try {
      await invoke('save_config_cmd', {
        config: {
          token,
          organization,
          project,
          refresh_interval: refreshInterval,
        },
      });
      onSave({ token, organization, project, refresh_interval: refreshInterval });
      setTestResult({ type: 'success', message: '配置已保存' });
      setTimeout(() => onClose(), 500);
    } catch (e: any) {
      setTestResult({ type: 'error', message: String(e) });
    }
  };

  const formatNumber = (n: number | undefined): string => {
    if (n === undefined || n === null) return 'N/A';
    if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
    if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`;
    return n.toString();
  };

  return (
    <div className={styles['config-panel']}>
      <div className={styles['panel-header']}>
        <h1 className={styles['panel-title']}>GLM 用量配置</h1>
        <button className={styles['close-btn']} onClick={onClose}>
          ✕
        </button>
      </div>

      <div className={styles['form-section']}>
        <div className={styles['form-group']}>
          <label htmlFor="token">Authorization Token</label>
          <input
            id="token"
            type="password"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder="从浏览器 Cookie 复制"
          />
          <span className={styles['help-text']}>
            从浏览器 Cookie 中的 bigmodel_token_production 复制
          </span>
        </div>

        <div className={styles['form-group']}>
          <label htmlFor="org">Organization ID</label>
          <input
            id="org"
            value={organization}
            onChange={(e) => setOrganization(e.target.value)}
            placeholder="例如: org-xxxxxxxx"
          />
          <span className={styles['help-text']}>从请求头 bigmodel-organization 复制</span>
        </div>

        <div className={styles['form-group']}>
          <label htmlFor="project">Project ID</label>
          <input
            id="project"
            value={project}
            onChange={(e) => setProject(e.target.value)}
            placeholder="例如: proj-xxxxxxxx"
          />
          <span className={styles['help-text']}>从请求头 bigmodel-project 复制</span>
        </div>

        <div className={styles['form-group']}>
          <label htmlFor="interval">刷新间隔</label>
          <select
            id="interval"
            value={refreshInterval}
            onChange={(e) => setRefreshInterval(Number(e.target.value))}
          >
            <option value={30}>30 秒</option>
            <option value={60}>60 秒 (推荐)</option>
            <option value={120}>120 秒</option>
            <option value={300}>300 秒</option>
          </select>
        </div>
      </div>

      {testResult && (
        <div className={clsx(styles.alert, styles[testResult.type])}>
          <span className={styles['alert-icon']}>
            {testResult.type === 'success' ? '✓' : '⚠'}
          </span>
          <span className={styles['alert-message']}>{testResult.message}</span>
        </div>
      )}

      <div className={styles.actions}>
        <button className={styles['btn-primary']} onClick={save}>
          保存配置
        </button>
        <button
          className={styles['btn-secondary']}
          onClick={testConnection}
          disabled={testing}
        >
          {testing ? '测试中...' : '测试连接'}
        </button>
      </div>
    </div>
  );
}
