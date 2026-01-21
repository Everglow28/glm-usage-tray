# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **前端框架从 Svelte 迁移到 React 18**
  - 改善响应式更新行为
  - 统一事件监听器管理
  - 修复自动刷新时界面更新问题
- **TypeScript 类型系统完善**
  - 移除所有 `any` 类型
  - Rust serde 字段映射改为驼峰式命名
  - 修复 Tauri `UnlistenFn` 类型定义
- **跨平台构建支持**
  - 添加 Linux (x86_64) 平台支持
  - 添加 macOS (Intel + Apple Silicon) 平台支持
  - GitHub Actions 自动构建所有平台安装包

### Fixed
- 修复模型使用详情数字显示为 0.0 的问题（正确显示次数）
- 修复 Tauri 2.x 权限配置（使用 capabilities 系统）
- 修复事件监听器资源泄漏问题

### Build
- 移除 `pnpm-lock.yaml` 版本跟踪

## [0.1.0] - 2025-01-21

### Added
- 系统托盘实时显示 GLM Coding Plan 使用量百分比
- 可配置刷新间隔（30/60/120/300 秒）
- 托盘菜单：刷新、配置面板、退出
- 详细用量显示界面（已用/总计/剩余/重置时间）
- API 凭证配置界面（Token、Organization、Project）
- 调试模式支持（通过 `GLM_DEBUG` 环境变量启用）
- 本地配置持久化存储

### Features
- 实时用量监控
- 自动后台定时刷新
- Windows 系统托盘集成
- 友好的中文界面

[Unreleased]: https://github.com/Everglow28/glm-usage-tray/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Everglow28/glm-usage-tray/releases/tag/v0.1.0
