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

    // 解析图标路径 - 兼容开发和生产环境
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("无法获取可执行文件目录")?;

    // 尝试多种可能的图标路径
    let icon_path = {
        // 开发环境: src-tauri/target/debug -> icons/
        let dev_path = exe_dir
            .parent()  // target
            .and_then(|p| p.parent())  // src-tauri
            .map(|p| p.join("icons/icon.png"));

        // 生产环境: .exe 同目录下的 icons/
        let prod_path = exe_dir.join("icons/icon.png");

        // 优先尝试生产环境路径，再尝试开发环境路径
        prod_path
            .exists()
            .then(|| prod_path)
            .or(dev_path.filter(|p| p.exists()))
            .ok_or("图标文件不存在")?
    };
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
