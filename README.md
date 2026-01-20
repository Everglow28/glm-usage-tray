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
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build
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
- Svelte 5 + TypeScript
- 系统 API: 智谱 AI 监控接口
