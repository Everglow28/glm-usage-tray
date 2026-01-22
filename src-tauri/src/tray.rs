use std::path::Path;

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

pub fn create_tray(app: &mut tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let open_config = MenuItem::with_id(app, "open", "打开配置", true, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "刷新用量", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_config, &refresh, &separator, &quit])?;

    // 解析图标路径
    // 在开发环境，可执行文件位于 src-tauri/target/debug/
    // 需要向上一级找到 src-tauri，然后进入 icons/
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("无法获取可执行文件目录")?;

    // 从 target/debug 向上找 src-tauri，然后进入 icons
    let icon_path = exe_dir
        .parent()  // target
        .and_then(|p| p.parent())  // src-tauri
        .map(|p| p.join("icons/icon.png"))
        .filter(|p| p.exists())
        .ok_or("图标文件不存在，请确保 src-tauri/icons/icon.png 存在")?;
    let icon = Image::from_path(icon_path)?;

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("GLM Usage Monitor")
        .icon_as_template(true)
        .icon(icon)
        .build(app)?;

    // 处理托盘事件
    let app_handle = app.handle().clone();
    tray.on_tray_icon_event(move |_tray, event| {
        if let TrayIconEvent::Click { button, .. } = event {
            if button == MouseButton::Left {
                let _ = app_handle.emit("tray-click", ());
            }
        }
    });

    // 处理菜单事件
    let app_handle = app.handle().clone();
    app.on_menu_event(move |app, event| {
        match event.id.as_ref() {
            "open" => {
                if let Some(window) = app.get_webview_window("config") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "refresh" => {
                let _ = app_handle.emit("manual-refresh", ());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    });

    Ok(tray)
}
