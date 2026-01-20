# GLM Usage Tray - 项目规范

## 项目概述

Windows 系统托盘应用，使用 Rust + Tauri 技术栈，实时监控 GLM Coding Plan 订阅使用量。

## 技术栈

| 组件 | 技术 |
|------|------|
| 后端 | Rust + Tauri 2.x |
| 前端 | Svelte 5 + TypeScript |
| API | 智谱 AI 监控接口 |
| 存储 | 本地 JSON 配置文件 |

## 目录结构

```
glm-usage-tray/
├── src-tauri/
│   ├── Cargo.toml              # Rust 依赖配置
│   ├── tauri.conf.json         # Tauri 配置
│   ├── src/
│   │   ├── main.rs             # 主入口
│   │   ├── lib.rs              # 库入口
│   │   ├── api.rs              # GLM API 调用
│   │   ├── tray.rs             # 系统托盘实现
│   │   ├── commands.rs         # Tauri 命令
│   │   ├── config.rs           # 配置管理
│   │   └── tasks.rs            # 后台定时任务
│   └── icons/
│       └── icon.png            # 托盘图标
├── src/
│   ├── App.svelte              # 主组件
│   ├── components/
│   │   ├── ConfigPanel.svelte  # 配置面板
│   │   └── UsageDisplay.svelte # 用量显示
│   └── main.ts
├── package.json
└── CLAUDE.md
```

## 开发规范

### 代码风格

* Rust：使用 `rustfmt` 默认配置
* TypeScript/Svelte：使用 ESLint + Prettier
* 命名：Rust 使用 snake_case，前端使用 camelCase

### API 配置

**端点**：`https://bigmodel.cn/api/monitor/usage/quota/limit`

**请求头**：
- `authorization`: JWT Bearer Token
- `bigmodel-organization`: 组织 ID
- `bigmodel-project`: 项目 ID

### 配置存储位置

- Windows: `%APPDATA%/glm-usage-tray/config.json`
- 存储内容：token, organization, project, refresh_interval

### 定时刷新

- 默认间隔：60 秒
- 可选：30/60/120/300 秒

## 工作流程

1. 修改代码前先阅读相关文件
2. 遵循单一职责原则
3. 新功能添加前考虑测试策略
4. 提交前确保代码编译通过
