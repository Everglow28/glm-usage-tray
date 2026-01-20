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

采用单一开发分支工作流：

1. **主分支保护**
   - `master` 分支禁止直接提交代码
   - 所有代码修改必须通过 `dev` 分支进行

2. **开发流程**
   ```bash
   # 切换到 dev 分支
   git checkout dev

   # 开发并提交
   git add .
   git commit -m "提交说明"
   ```

3. **合并到主分支**
   ```bash
   # 切换到主分支
   git checkout master

   # 合并 dev 分支
   git merge dev --ff-only
   ```

4. **分支规则**
   - 仅保留两个分支：`master` 和 `dev`
   - 所有开发（功能、修复、文档）都在 `dev` 上进行
   - 定期将 `dev` 合并到 `master`

### 代码开发

1. 修改代码前先阅读相关文件
2. 遵循单一职责原则
3. 新功能添加前考虑测试策略
4. 提交前确保代码编译通过

## 重要注意事项

### Rust 字段序列化规则

Rust 结构体使用 `#[serde(rename = "...")]` 序列化后，JSON 字段名为驼峰式：

| Rust 字段名 | serde 别名 | JSON 字段名 |
|------------|-----------|------------|
| `limit_type` | `rename = "type"` | `type` |
| `current_value` | `rename = "currentValue"` | `currentValue` |
| `usage_details` | `rename = "usageDetails"` | `usageDetails` |
| `next_reset_time` | `rename = "nextResetTime"` | `nextResetTime` |

**前端必须使用驼峰式字段名**（如 `currentValue`），而非 Rust 的 snake_case（如 `current_value`）。

### Svelte 响应式更新

当需要在函数内更新变量并触发界面重新渲染时：
- 使用响应式语句 `$: ...` 确保依赖被正确追踪
- 对于复杂的更新逻辑，可使用计数器变量强制刷新

示例：
```javascript
let refreshCounter = 0;
let lastUpdate: Date | null = null;

$: displayTime = (() => {
  const _ = refreshCounter; // 确保 refreshCounter 变化时触发重新计算
  if (!lastUpdate) return "未更新";
  return formatDate(lastUpdate);
})();
```

## 已修复问题记录

### 字段名不匹配导致显示错误
- **问题**：前端使用 `current_value`，后端序列化为 `currentValue`
- **修复**：统一使用驼峰式字段名
- **文件**：`src/components/ConfigPanel.svelte`, `src/components/UsageDisplay.svelte`

### 首次加载不自动刷新
- **问题**：从配置界面进入使用情况界面时，数据不加载
- **修复**：在 `UsageDisplay` 组件 `onMount` 时检查数据，若无则触发刷新
- **文件**：`src/components/UsageDisplay.svelte`

### 刷新按钮点击后时间不更新
- **问题**：响应式语句与手动刷新冲突，`lastUpdate` 被覆盖
- **修复**：使用 `isManualRefresh` 标志和 `refreshCounter` 计数器
- **文件**：`src/components/UsageDisplay.svelte`
