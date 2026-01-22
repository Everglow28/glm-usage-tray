use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    Emitter, Image, Manager,
};

pub fn create_tray(app: &mut tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let open_config = MenuItem::with_id(app, "open", "打开配置", true, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "刷新用量", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_config, &refresh, &separator, &quit])?;

    let icon_path = app.path().resolve("icons/icon.png", tauri::path::BaseDirectory::Resource)?;
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
