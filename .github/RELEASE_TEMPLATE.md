## 🎉 版本 x.y.z

### 📦 下载

**Windows**: `.exe` / `.msi` 安装包
**macOS**: `.dmg` 磁盘映像（Apple Silicon）

### 🚀 快速安装

- **Windows**: 双击安装文件
- **macOS**: 打开 `.dmg`，拖应用到应用程序文件夹

### ⚙️ 配置 API 凭证

首次运行需要配置 GLM API 信息：

1. 浏览器访问 [智谱 AI 用量页面](https://bigmodel.cn/usercenter/glm-coding/usage)
2. F12 打开开发者工具 → Network 标签
3. 复制请求中的信息：
   - Cookie 中的 `bigmodel_token_production`
   - 请求头 `bigmodel-organization`
   - 请求头 `bigmodel-project`

### 🔧 调试

设置环境变量 `GLM_DEBUG=1` 启动应用查看详细日志。

---

更多信息：[README](https://github.com/Everglow28/glm-usage-tray) | [CHANGELOG](https://github.com/Everglow28/glm-usage-tray/blob/master/CHANGELOG.md)
