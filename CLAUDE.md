# GLM Usage Tray - 项目规范

## 项目概述

Windows 系统托盘应用，使用 Rust + Tauri 技术栈，实时监控 GLM Coding Plan 订阅使用量。

## 技术栈

| 组件 | 技术 |
|------|------|
| 后端 | Rust + Tauri 2.x |
| 前端 | Svelte 4 + TypeScript |
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
│   │   ├── lib.rs              # 库入口 + 日志宏
│   │   ├── api.rs              # GLM API 调用
│   │   ├── debug.rs            # 调试开关
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

### Git 分支管理

采用功能分支工作流，确保主分支稳定性：

1. **主分支保护**
   - `master` 分支禁止直接提交代码
   - 所有代码修改必须通过功能分支进行

2. **功能分支开发**
   ```bash
   # 创建功能分支
   git checkout -b feature/功能名称

   # 开发并提交
   git add .
   git commit -m "提交说明"
   ```

3. **合并到主分支**
   ```bash
   # 切换到主分支
   git checkout master

   # 合并功能分支
   git merge feature/功能名称

   # 删除已完成的功能分支（可选）
   git branch -d feature/功能名称
   ```

4. **分支命名规范**
   - 功能开发：`feature/功能描述`
   - 问题修复：`fix/问题描述`
   - 文档更新：`docs/文档描述`

### 代码开发

1. 修改代码前先阅读相关文件
2. 遵循单一职责原则
3. 新功能添加前考虑测试策略
4. 提交前确保代码编译通过
