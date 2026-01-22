# GLM Usage Tray - 项目规范

## 记忆文件存储

项目记忆文件存放在 `.memory/` 文件夹中（本地文件，不提交到 Git）：
- `WORKFLOW.md` - 开发工作流指南
- `RELEASE.md` - 发布流程

---

## 项目概述

Windows 系统托盘应用，使用 Rust + Tauri 技术栈，实时监控 GLM Coding Plan 订阅使用量。

---

## 技术栈

| 组件 | 技术 |
|------|------|
| 后端 | Rust + Tauri 2.x |
| 前端 | React 18 + TypeScript |
| API | 智谱 AI 监控接口 |
| 存储 | 本地 JSON 配置文件 |

---

## 目录结构

```
glm-usage-tray/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs, lib.rs, api.rs
│   │   ├── tray.rs, commands.rs
│   │   └── config.rs, tasks.rs
│   └── Cargo.toml, tauri.conf.json
├── src/
│   ├── App.tsx
│   ├── components/
│   │   ├── ConfigPanel.tsx
│   │   └── UsageDisplay.tsx
│   ├── hooks/
│   │   ├── useConfig.ts
│   │   └── useGlmUsage.ts
│   └── types/api.ts
├── package.json
├── vite.config.ts
└── CLAUDE.md
```

---

## 开发规范

### 代码风格

- **Rust**：使用 `rustfmt` 默认配置
- **TypeScript/React**：使用 ESLint + Prettier
- **命名**：Rust 使用 `snake_case`，前端使用 `camelCase`

### API 配置

**端点**：`https://bigmodel.cn/api/monitor/usage/quota/limit`

**请求头**：
- `authorization`: JWT Bearer Token
- `bigmodel-organization`: 组织 ID
- `bigmodel-project`: 项目 ID

### 配置存储

- **Windows**: `%APPDATA%/glm-usage-tray/config.json`
- **存储内容**：token, organization, project, refresh_interval

### 定时刷新

- **默认间隔**：60 秒
- **可选间隔**：30/60/120/300 秒

---

## 重要注意事项

### Rust 字段序列化规则

Rust 结构体使用 `#[serde(rename = "...")]` 序列化后，JSON 字段名为驼峰式：

| Rust 字段名 | serde 别名 | JSON 字段名 |
|------------|-----------|------------|
| `limit_type` | `rename = "type"` | `type` |
| `current_value` | `rename = "currentValue"` | `currentValue` |
| `usage_details` | `rename = "usageDetails"` | `usageDetails` |
| `next_reset_time` | `rename = "nextResetTime"` | `nextResetTime` |

**前端必须使用驼峰式字段名**（如 `currentValue`），而非 Rust 的 `snake_case`（如 `current_value`）。

### React Hooks 注意事项

1. **Tauri `listen()` 资源管理**：必须保存 unlisten 函数并在 `useEffect` cleanup 中调用，避免内存泄漏

```tsx
const unlistensRef = useRef<(() => Promise<void>)[]>([]);

useEffect(() => {
  listen("event", handler).then(unlisten =>
    unlistensRef.current.push(unlisten)
  );
  return () => {
    unlistensRef.current.forEach(unlisten => unlisten());
  };
}, []);
```

2. **依赖管理**：确保 useEffect 依赖数组正确，避免无限循环

---

## 工作流程

详细的开发工作流请参考 [`.memory/WORKFLOW.md`](.memory/WORKFLOW.md)

### 分支策略

```
master (主分支)
  ↑ Release Workflow
dev (集成验证，CI 保护)
  ↑ 仅 PR 合并 (rebase)
feature/xxx, fix/xxx
```

### 核心规则

- 所有代码修改必须在 `feature/xxx` 或 `fix/xxx` 分支完成
- 通过 PR 合并到 `dev`，必须通过 CI 检查（install, build, lint）
- `dev` 分支禁止直接 push
- PR 合并方式为 rebase
- 打包发布由 Release Workflow 负责，仅在 master 分支触发
