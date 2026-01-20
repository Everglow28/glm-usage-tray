# GLM Usage Tray

Windows 系统托盘应用，实时监控 GLM Coding Plan 订阅使用量。

## 功能

- 系统托盘实时显示当前用量百分比
- 可配置刷新间隔（30/60/120/300 秒）
- 托盘菜单：刷新、配置、退出
- 详细用量显示（已用/总计/剩余）

## 开发

```bash
# 安装依赖
pnpm install

# 开发模式（正常模式，无 debug 日志）
pnpm run devT

# 开发模式（调试模式，显示详细日志）
pnpm run devT:debug

# 构建
pnpm run build
```

## 调试模式

调试模式下会输出详细的 API 请求/响应日志，便于排查问题。

### 方法 1: 使用预定义脚本

```bash
pnpm run devT:debug
```

### 方法 2: 直接设置环境变量（跨平台）

```bash
# Windows CMD
cross-env GLM_DEBUG=1 pnpm run devT

# Windows PowerShell
$env:GLM_DEBUG="1"; pnpm run devT

# Linux/macOS
GLM_DEBUG=1 pnpm run devT
```

### 支持的环境变量

以下任一环境变量都可启用调试模式：

| 环境变量 | 值 |
|----------|-----|
| `GLM_DEBUG` | `1`, `true`, `debug` |
| `DEBUG` | `1`, `true`, `debug` |
| `RUST_LOG` | `debug`, `glm_usage_tray` |

### 示例

```bash
# 使用 DEBUG 变量
cross-env DEBUG=1 pnpm run devT

# 使用 RUST_LOG 变量
cross-env RUST_LOG=debug pnpm run devT

# 组合多个值
cross-env GLM_DEBUG=true pnpm run devT
```

## 配置说明

首次运行需要配置 API 凭证：

1. **Authorization Token**: 从浏览器 Cookie 中的 `bigmodel_token_production` 复制
2. **Organization ID**: 从浏览器请求头 `bigmodel-organization` 复制
3. **Project ID**: 从浏览器请求头 `bigmodel-project` 复制

## 项目结构

```
glm-usage-tray/
├── src-tauri/          # Rust 后端
│   ├── src/            # 源代码
│   └── icons/          # 图标资源
├── src/                # Svelte 前端
└── package.json
```

## 技术栈

- Rust + Tauri 2.x
- Svelte 4 + TypeScript
- 系统 API: 智谱 AI 监控接口
