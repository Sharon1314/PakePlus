// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Manager
};

fn main() {
  // 1. 定义托盘菜单项
  let show = CustomMenuItem::new("show".to_string(), "显示窗口");
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  // 2. 创建 SystemTray 对象并附加菜单
  let system_tray = SystemTray::new().with_menu(tray_menu);

  // 3. 用 Builder 取代原来的 run()，把托盘和事件注册进来
  tauri::Builder::default()
    // 如果你还有 invoke_handler、setup 等，在这里链上去就行
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "show" => {
            // "显示窗口"：找到名为 "main" 的窗口并唤起
            if let Some(w) = app.get_window("main") {
              let _ = w.show();
              let _ = w.set_focus();
            }
          }
          "quit" => {
            // "退出"：直接结束进程
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    // 这一行会去读 tauri.conf.json
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
