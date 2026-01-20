/// 检查是否启用调试日志
/// 通过设置环境变量 GLM_DEBUG=1 或 DEBUG=1 或 RUST_LOG=debug 来启用
pub fn is_debug_enabled() -> bool {
    std::env::var("GLM_DEBUG")
        .or_else(|_| std::env::var("DEBUG"))
        .or_else(|_| std::env::var("RUST_LOG"))
        .map(|v| {
            let v = v.to_lowercase();
            v == "1" || v == "true" || v == "debug" || v.contains("glm_usage_tray")
        })
        .unwrap_or(false)
}
