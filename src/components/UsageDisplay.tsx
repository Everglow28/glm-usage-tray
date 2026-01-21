import { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import clsx from 'clsx';
import type { UsageData } from '../types/api';
import styles from './UsageDisplay.module.css';

interface UsageDisplayProps {
  usage: UsageData | null;
  error: string | null;
  onConfig: () => void;
}

interface Limit {
  type: string;
  percentage: number;
  currentValue: number;
  usage: number;
  remaining: number;
  usage_details?: { model_code: string; usage: number }[];
  nextResetTime?: string;
}

export default function UsageDisplay({ usage, error, onConfig }: UsageDisplayProps) {
  const [lastUpdate, setLastUpdate] = useState<Date | null>(null);
  const [isManualRefresh, setIsManualRefresh] = useState(false);

  const hasData = usage?.success === true && Array.isArray(usage?.data?.limits) && usage.data.limits.length > 0;
  const isLoading = !hasData && !error;

  const limits = useMemo(() => {
    return usage?.data?.limits || [];
  }, [usage]);

  // 监听 usage 变化，提取 limits 数据
  useEffect(() => {
    if (usage?.success && usage?.data?.limits) {
      // 只有在非手动刷新时才更新 lastUpdate
      if (!isManualRefresh) {
        setLastUpdate(new Date());
      }
    }
  }, [usage, isManualRefresh]);

  // 组件挂载时，如果没有数据，自动触发一次刷新
  useEffect(() => {
    const hasInitialData = usage?.success === true && Array.isArray(usage?.data?.limits) && usage.data.limits.length > 0;
    if (!hasInitialData && !error) {
      refresh();
    }
  }, []);

  const refresh = async () => {
    setIsManualRefresh(true);
    try {
      const result = await invoke('manual_refresh');
      if (result) {
        setLastUpdate(new Date());
      }
    } catch (e: any) {
      // Error handling done in App component
    } finally {
      setTimeout(() => {
        setIsManualRefresh(false);
      }, 0);
    }
  };

  const formatNumber = (num: number | undefined, limitType: string): string => {
    if (num === undefined || num === null) return 'N/A';
    if (limitType === 'TOKENS_LIMIT') {
      return (num / 10_000).toFixed(1);
    }
    return num.toString();
  };

  const getLimitTitle = (limit: Limit): string => {
    switch (limit.type) {
      case 'TIME_LIMIT':
        return 'MCP每月额度';
      case 'TOKENS_LIMIT':
        return '每5小时使用限额';
      default:
        return limit.type;
    }
  };

  const getLimitTypeLabel = (limit: Limit): string => {
    switch (limit.type) {
      case 'TIME_LIMIT':
        return '次';
      case 'TOKENS_LIMIT':
        return '万';
      default:
        return limit.type;
    }
  };

  const getResetTimeText = (limit: Limit): string => {
    if (limit.type === 'TIME_LIMIT') {
      return '每月1号00:00重置';
    }
    if (limit.type === 'TOKENS_LIMIT') {
      if (limit.nextResetTime) {
        const date = new Date(limit.nextResetTime);
        return `重置时间: ${formatDateTime(date)}`;
      }
      return '重置时间: 未知';
    }
    return '重置时间: 未知';
  };

  const formatDate = (date: Date): string => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  };

  const formatDateTime = (date: Date): string => {
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    return `${hours}:${minutes}:${seconds}`;
  };

  const displayTime = useMemo(() => {
    if (!lastUpdate) return '未更新';
    return formatDate(lastUpdate);
  }, [lastUpdate]);

  const getProgressColor = (percentage: number): string => {
    if (percentage >= 90) return '#dc2626';
    if (percentage >= 70) return '#f59e0b';
    return '#2563eb';
  };

  return (
    <div className={styles['usage-display']}>
      {/* 顶部栏 */}
      <div className={styles['top-bar']}>
        <div className={styles['title-section']}>
          <h1 className={styles['app-title']}>GLM 用量监控</h1>
          <div className={styles['status-dot']}></div>
        </div>
        <button className={styles['config-btn']} onClick={onConfig}>
          <span className={styles.icon}>⚙</span> 配置
        </button>
      </div>

      {error && (
        <div className={styles['error-banner']}>
          <span className={styles['error-icon']}>⚠</span>
          <span className={styles['error-text']}>{error}</span>
        </div>
      )}

      {hasData ? (
        <>
          <div className={styles['cards-grid']}>
            {limits.map((limit: Limit) => (
              <div key={limit.type} className={styles['limit-card']}>
                {/* 卡片标题 */}
                <div className={styles['card-header']}>
                  <h2 className={styles['card-title']}>{getLimitTitle(limit)}</h2>
                  <span className={styles['type-unit']}>{getLimitTypeLabel(limit)}</span>
                </div>

                {/* 百分比显示 */}
                <div className={styles['percentage-display']}>
                  <span className={styles['percentage-value']}>{limit.percentage.toFixed(0)}%</span>
                  <span className={styles['percentage-label']}>已使用</span>
                </div>

                {/* 进度条 */}
                <div className={styles['progress-container']}>
                  <div className={styles['progress-track']}>
                    <div
                      className={styles['progress-fill']}
                      style={{
                        width: `${limit.percentage}%`,
                        backgroundColor: getProgressColor(limit.percentage),
                      }}
                    ></div>
                  </div>
                </div>

                {/* 数值详情 */}
                <div className={styles['values-row']}>
                  <div className={styles['value-item']}>
                    <span className={styles['value-label']}>已用</span>
                    <span className={styles['value-number']}>
                      {formatNumber(limit.currentValue, limit.type)}
                    </span>
                  </div>
                  <div className={styles['value-divider']}>/</div>
                  <div className={styles['value-item']}>
                    <span className={styles['value-label']}>总额</span>
                    <span className={styles['value-number']}>
                      {formatNumber(limit.usage, limit.type)}
                    </span>
                  </div>
                  <div className={styles['value-item']}>
                    <span className={styles['value-label']}>剩余</span>
                    <span className={styles['value-number']}>
                      {formatNumber(limit.remaining, limit.type)}
                    </span>
                  </div>
                </div>

                {/* 重置时间 */}
                <div className={styles['reset-info']}>
                  <span className={styles['reset-icon']}>↻</span>
                  <span className={styles['reset-text']}>{getResetTimeText(limit)}</span>
                </div>

                {/* 使用详情 */}
                {limit.usage_details && limit.usage_details.length > 0 && (
                  <div className={styles['details-section']}>
                    <h3 className={styles['details-title']}>模型使用详情</h3>
                    <div className={styles['details-list']}>
                      {limit.usage_details.map((detail, index) => (
                        <div key={index} className={styles['detail-row']}>
                          <span className={styles['detail-name']}>{detail.model_code}</span>
                          <span className={styles['detail-value']}>
                            {formatNumber(detail.usage, 'TOKENS_LIMIT')}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* 底部栏 */}
          <div className={styles['bottom-bar']}>
            <div className={styles['update-info']}>
              <span className={styles['update-icon']}>◷</span>
              <span className={styles['update-label']}>最近更新时间: </span>
              <span className={styles['update-text']}>{displayTime}</span>
            </div>
            <button className={styles['refresh-btn']} onClick={refresh}>
              <span className={styles['refresh-icon']}>⟳</span> 刷新
            </button>
          </div>
        </>
      ) : (
        <div className={styles['loading-state']}>
          <div className={styles['loading-spinner']}></div>
          <p className={styles['loading-text']}>
            {isLoading ? '正在加载用量数据...' : '暂无数据，请先配置 API 信息'}
          </p>
          <button className={styles['loading-btn']} onClick={refresh}>
            手动刷新
          </button>
        </div>
      )}
    </div>
  );
}
