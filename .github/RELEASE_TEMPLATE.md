## 🎉 版本 x.y.z

> **注意**：此文件为 Release 说明模板，创建 Release 时请将 `x.y.z` 替换为实际版本号（如 `0.1.0`）

### 📦 下载

| 文件 | 说明 |
|------|------|
| `glm-usage-tray_x.y.z_x64-setup.exe` | Windows 安装程序 |
| `glm-usage-tray_x.y.z_x64-setup.msi` | Windows MSI 安装包 |

### 🚀 安装方法

#### 方法 1: 使用 .exe 安装包
1. 下载 `.exe` 文件
2. 双击运行
3. 按照安装向导完成安装

#### 方法 2: 使用 .msi 安装包
1. 下载 `.msi` 文件
2. 双击运行
3. 按照安装向导完成安装

### 📝 配置说明

首次运行需要配置 GLM API 凭证：

1. 打开浏览器开发者工具 (F12)
2. 访问 [智谱 AI 开放平台](https://open.bigmodel.cn/)
3. 登录后在 Network 标签找到 API 请求
4. 复制以下信息：
   - **Authorization Token**: Cookie 中的 `bigmodel_token_production`
   - **Organization ID**: 请求头 `bigmodel-organization`
   - **Project ID**: 请求头 `bigmodel-project`

### 🔧 调试模式

如需查看详细日志，设置环境变量 `GLM_DEBUG=1` 后启动应用。

### 📚 更多信息

- [项目主页](https://github.com/Everglow28/glm-usage-tray)
- [完整文档](https://github.com/Everglow28/glm-usage-tray/blob/master/README.md)
- [更新日志](https://github.com/Everglow28/glm-usage-tray/blob/master/CHANGELOG.md)
