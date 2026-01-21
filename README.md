# GLM Usage Tray

跨平台系统托盘应用，实时监控 GLM Coding Plan 订阅使用量。

支持 Windows、macOS (Intel x64)。

## 功能

- 系统托盘实时显示当前用量百分比
- 可配置刷新间隔（30/60/120/300 秒）
- 托盘菜单：刷新、配置、退出
- 详细用量显示（已用/总计/剩余/模型使用详情）

## 下载安装

访问 [Releases](https://github.com/Everglow28/glm-usage-tray/releases) 下载对应平台的安装包：

| 平台 | 安装包 |
|------|--------|
| Windows | `.exe` / `.msi` |
| macOS | `.dmg` (Intel x64) |

## 配置说明

首次运行需要配置 API 凭证：

1. 浏览器访问 [智谱 AI 用量页面](https://bigmodel.cn/usercenter/glm-coding/usage)
2. 按 `F12` 打开开发者工具 → Network 标签
3. 刷新页面，找到 API 请求
4. 复制以下信息：
   - **Authorization Token**: Cookie 中的 `bigmodel_token_production`
   - **Organization ID**: 请求头 `bigmodel-organization`
   - **Project ID**: 请求头 `bigmodel-project`

## 开发

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm run dev

# 开发模式（调试）
pnpm run dev:debug

# 构建
pnpm run build
```

### 调试模式

启用详细日志输出：

| 方法 | 命令 |
|------|------|
| 脚本 | `pnpm run dev:debug` |
| 环境变量 | `GLM_DEBUG=1 pnpm run dev` |

支持的环境变量：`GLM_DEBUG`、`DEBUG`、`RUST_LOG`

## 项目结构

```
glm-usage-tray/
├── src-tauri/          # Rust 后端
│   ├── src/            # 源代码
│   │   ├── api.rs      # GLM API 调用
│   │   ├── tray.rs     # 系统托盘
│   │   ├── tasks.rs    # 定时任务
│   │   └── ...
│   ├── icons/          # 图标资源
│   └── capabilities/   # Tauri 权限配置
├── src/                # React 前端
│   ├── components/     # React 组件
│   ├── hooks/          # 自定义 Hooks
│   ├── types/          # TypeScript 类型
│   └── main.tsx        # 入口
└── package.json
```

## 技术栈

- **后端**: Rust + Tauri 2.x
- **前端**: React 18 + TypeScript
- **构建**: Vite 5
- **样式**: CSS Modules
- **API**: 智谱 AI 监控接口

## License

[MIT](LICENSE)
