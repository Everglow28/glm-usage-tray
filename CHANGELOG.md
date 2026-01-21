# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
