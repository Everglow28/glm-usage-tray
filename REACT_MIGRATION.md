# React 迁移记忆文档

> 记录 Svelte → React 迁移过程中的决策、问题和解决方案
> 基于 React 最佳实践和 Tauri 开发经验

## 迁移背景

**迁移原因**：Svelte 响应式系统在处理自动刷新更新时存在不可预测的行为（需手动维护 `refreshCounter` 强制刷新）

**目标**：
- 提高状态更新的可预测性
- 更好的调试工具支持
- 更广泛的技术栈生态

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | React 18 + TypeScript |
| 构建 | Vite |
| 样式 | CSS Modules + clsx |
| 工具库 | @tauri-apps/api (不变) |
| 后端 | Tauri 2.x (不变) |

---

## 组件映射表

| Svelte 组件 | React 组件 | 行数 | 复杂度 |
|-------------|------------|------|--------|
| `App.svelte` | `App.tsx` | 73 | 低 |
| `ConfigPanel.svelte` | `components/ConfigPanel.tsx` | 344 | 中 |
| `UsageDisplay.svelte` | `components/UsageDisplay.tsx` | 651 | 中高 |
| - | `hooks/useGlmUsage.ts` | - | 新增 |

---

## TypeScript 类型定义

根据智谱 API 返回结构定义接口（替代 `any`）：

```tsx
// types/api.ts
export interface UsageDetail {
  model_code: string;
  usage: number;
}

export interface Limit {
  type: string;           // "TIME_LIMIT" | "TOKENS_LIMIT"
  percentage: number;
  current_value: number;
  usage: number;
  remaining: number;
  usage_details?: UsageDetail[];
  next_reset_time?: string;
}

export interface UsageData {
  success: true;
  data: {
    limits: Limit[];
  };
}

export interface UsageError {
  success: false;
  code: string;
  message: string;
}

export type UsageResponse = UsageData | UsageError;

export interface AppConfig {
  token: string;
  organization: string;
  project: string;
  refresh_interval: number;
}
```

---

## 架构优化：Custom Hooks

### 问题分析

Svelte 倾向于把逻辑写在 `<script>` 标签里，直接翻译会导致 React 组件臃肿，尤其是处理 Tauri 的 `listen` 和 `invoke`。

### 解决方案：封装 Custom Hooks

```tsx
// hooks/useGlmUsage.ts
import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UsageData } from '../types/api';

export function useGlmUsage() {
  const [usage, setUsage] = useState<UsageData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const unlistenRef = useRef<(() => Promise<void>) | null>(null);

  // 手动刷新
  const refresh = useCallback(async () => {
    try {
      const data = await invoke<UsageData>("manual_refresh");
      setUsage(data);
      setError(null);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  // 初始化和事件监听
  useEffect(() => {
    let mounted = true;

    // 获取初始数据
    invoke<UsageData>("get_current_usage").then(data => {
      if (mounted) {
        setUsage(data);
      }
    }).catch(e => {
      if (mounted) {
        setError(String(e));
      }
    });

    // 监听自动刷新事件
    listen("usage-update", (event: any) => {
      if (mounted) {
        setUsage(event.payload);
        setError(null);
      }
    }).then(unlisten => {
      if (mounted) {
        unlistenRef.current = unlisten;
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
```

```tsx
// hooks/useConfig.ts
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
```

---

## Tauri 特定注意事项

### 1. 生命周期与资源清理

**问题**：在开发模式下（Vite HMR），不清理 `listen` 会导致重复监听和内存泄漏。

```tsx
// ❌ 错误：未清理监听
useEffect(() => {
  listen("usage-update", (event) => {
    setUsage(event.payload);
  });
}, []);

// ✅ 正确：返回清理函数
useEffect(() => {
  let mounted = true;

  const setupListener = async () => {
    const unlisten = await listen("usage-update", (event) => {
      if (mounted) {
        setUsage(event.payload);
      }
    });

    // 保存 unlisten 以便清理
    return unlisten;
  };

  const unlistenPromise = setupListener();

  return () => {
    mounted = false;
    unlistenPromise.then(unlisten => unlisten());
  };
}, []);
```

### 2. useEffect 闭包陷阱

**问题**：在 `listen` 回调中引用组件状态时，状态可能过期。

```tsx
// ❌ 闭包陷阱：counter 永远是 0
const [counter, setCounter] = useState(0);
useEffect(() => {
  listen("event", () => {
    console.log(counter); // 始终打印初始值
  });
}, []); // 空依赖

// ✅ 方案1：添加依赖
useEffect(() => {
  const unlistenPromise = listen("event", () => {
    console.log(counter);
  });
  return () => { unlistenPromise.then(u => u()); };
}, [counter]);

// ✅ 方案2：使用 useRef（推荐）
const counterRef = useRef(counter);
counterRef.current = counter;

useEffect(() => {
  listen("event", () => {
    console.log(counterRef.current); // 始终获取最新值
  });
}, []);
```

---

## 关键语法转换

### 1. 双向绑定

**Svelte**:
```svelte
<script>
  let token = "";
</script>
<input bind:value={token} />
```

**React**:
```tsx
const [token, setToken] = useState("");
<input value={token} onChange={e => setToken(e.target.value)} />
```

---

### 2. 响应式语句 `$:` → `useEffect`

**Svelte**:
```svelte
<script>
  $: if (usage?.success) {
    limits = usage.data.limits;
    lastUpdate = new Date();
  }
</script>
```

**React**:
```tsx
useEffect(() => {
  if (usage?.data?.limits) {
    setLimits(usage.data.limits);
    setLastUpdate(new Date());
  }
}, [usage]);
```

---

### 3. 条件渲染

**Svelte**:
```svelte
{#if hasData}
  <div>内容</div>
{:else}
  <div>空状态</div>
{/if}
```

**React**:
```tsx
{hasData ? (
  <div>内容</div>
) : (
  <div>空状态</div>
)}
```

---

### 4. 列表渲染

**Svelte**:
```svelte
{#each limits as limit (limit.type)}
  <Card {limit} />
{/each}
```

**React**:
```tsx
{limits.map(limit => (
  <Card key={limit.type} limit={limit} />
))}
```

---

### 5. Props 传递

**Svelte**:
```svelte
<script>
  export let usage: any;
  export let onConfig: () => void;
</script>
```

**React**:
```tsx
interface Props {
  usage: UsageData | null;
  onConfig: () => void;
}

export default function UsageDisplay({ usage, onConfig }: Props) {
  // ...
}
```

---

### 6. 样式 + 条件类名

**Svelte**:
```svelte
<div class:progress-bar={true} class:danger={percent > 90}>
```

**React + CSS Modules + clsx**:
```tsx
import clsx from 'clsx';
import styles from './UsageDisplay.module.css';

<div className={clsx(
  styles['progress-bar'],
  percent > 90 && styles.danger
)}>
```

---

## 依赖变更

### 移除
```bash
npm uninstall @sveltejs/vite-plugin-svelte @tsconfig/svelte svelte svelte-check svelte-preprocess
```

### 安装
```bash
# React 核心
npm install react react-dom clsx

# 类型定义
npm install -D @types/react @types/react-dom

# Vite 插件
npm install -D @vitejs/plugin-react
```

---

## 文件结构变化

```
src/
├── main.ts                    # React 入口
├── vite-env.d.ts              # React 类型声明
├── App.tsx                    # 主应用（简化版）
├── App.css                    # 全局样式
├── types/
│   └── api.ts                 # TypeScript 类型定义
├── hooks/
│   ├── useGlmUsage.ts         # 用量数据 Hook
│   └── useConfig.ts           # 配置管理 Hook
├── components/
│   ├── ConfigPanel.tsx
│   ├── ConfigPanel.module.css
│   ├── UsageDisplay.tsx
│   └── UsageDisplay.module.css
└── app.css                    # 重置样式
```

---

## 已知问题和注意事项

### ✅ 迁移后可解决的问题

1. **自动刷新不更新界面** - React 的 `useEffect` 依赖数组更可靠
2. **需要 `refreshCounter` 强制刷新** - React 状态更新自动触发重渲染
3. **响应式语句难以调试** - React DevTools 支持完善
4. **组件逻辑臃肿** - Custom Hooks 解耦复用

### ⚠️ 需要注意的点

1. **Tauri listen 清理** - 必须在 useEffect 返回函数中调用 unlisten
2. **闭包陷阱** - listen 回调中的状态引用需要 useRef 或正确依赖
3. **条件类名** - 使用 clsx 替代 Svelte 的 `class:` 语法
4. **key 稳定性** - 列表渲染确保使用稳定的 key（如 `limit.type`）

---

## 验收标准

### 功能验收
- [ ] 所有功能正常：配置保存、连接测试、用量显示
- [ ] 自动刷新能正常更新界面
- [ ] 手动刷新正常工作
- [ ] 样式无差异
- [ ] 无控制台错误或警告
- [ ] Tauri 打包正常

### Tauri 专项验收
- [ ] **HMR 稳定性**：修改代码保存后，Tauri 窗口的监听事件依然唯一且有效（无重复打印日志）
- [ ] **Bundle Size**：React 引入后打包体积增加在可接受范围内（通常 +30-50KB）

### 代码质量验收
- [ ] 无 `any` 类型，全部使用定义的接口
- [ ] 所有 Tauri listen 都有清理逻辑
- [ ] 无 useEffect 闭包陷阱警告

---

## 迁移记录

### 2025-01-21 迁移完成

**状态**：迁移完成，开发环境启动成功

**完成项目**：
- ✅ 更新 package.json：移除 Svelte 依赖，添加 React 依赖
- ✅ 更新 vite.config.ts：替换为 React 插件
- ✅ 创建 TypeScript 类型定义 (types/api.ts)
- ✅ 创建 useGlmUsage Custom Hook
- ✅ 创建 useConfig Custom Hook
- ✅ 迁移 App.svelte → App.tsx
- ✅ 迁移 ConfigPanel.svelte → ConfigPanel.tsx
- ✅ 迁移 UsageDisplay.svelte → UsageDisplay.tsx
- ✅ 创建 CSS Modules 样式文件
- ✅ 更新 main.tsx React 入口文件和 index.html
- ✅ 更新 tsconfig.json 为 React 配置
- ✅ 删除旧的 Svelte 组件文件

**技术栈**：
- React 18.3.1
- TypeScript 5.6.0
- Vite 5.4.21
- clsx 2.1.1

**待修复问题**：
- 存在待修复的 bug（待具体记录）

**注意事项**：
- Git Bash 环境下使用 `taskkill` 需要通过 `cmd.exe //c` 调用
- 端口 1420 被占用时使用 `netstat -ano | findstr :1420` 查找进程
