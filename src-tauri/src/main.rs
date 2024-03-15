// This is probably the worst code I've ever written, but it works.
// You don't understand what I did in here? I don't understand it either.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Datelike, Local};
use std::sync::{Arc, Mutex};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let date = Local::now().format("%a, %b %d");
    let today = CustomMenuItem::new("today", date.to_string()).disabled();
    let quit =
        CustomMenuItem::new("quit".to_string(), "Quit Amie completely").accelerator("CmdOrCtrl+Q");
    let hide = CustomMenuItem::new("hide".to_string(), "Close Amie");
    let tray_menu = SystemTrayMenu::new()
        .add_item(today)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    let window_visible = Arc::new(Mutex::new(true));

    let window_visible_for_tray = window_visible.clone();

    tauri::Builder::default()
        .system_tray(system_tray)
        .setup(|app| {
            let app_handle = app.handle();
            let tray_handle = app_handle.tray_handle();

            let day = Local::now().day();
            let icon = tauri::Icon::File(format!("icons/day/{}.png", day).into());
            tray_handle.set_icon(icon).map_err(|e| e.into())
        })
        .on_system_tray_event(move |app, event| {
            let window_visible = window_visible_for_tray.clone();
            match event {
                SystemTrayEvent::LeftClick { .. } => {
                    let window = app.get_window("main").unwrap();
                    let mut visible = window_visible.lock().unwrap();
                    if *visible {
                        window.hide().unwrap();
                        *visible = false;
                    } else {
                        window.show().unwrap();
                        *visible = true;
                    }
                    let item_handle = app.tray_handle().get_item("hide");
                    if *visible {
                        item_handle.set_title("Close Amie").unwrap();
                    } else {
                        item_handle.set_title("Open Amie").unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "hide" => {
                            let window = app.get_window("main").unwrap();
                            let mut visible = window_visible.lock().unwrap();
                            if *visible {
                                window.hide().unwrap();
                                *visible = false;
                                item_handle.set_title("Open Amie").unwrap();
                            } else {
                                window.show().unwrap();
                                *visible = true;
                                item_handle.set_title("Close Amie").unwrap();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .on_window_event(move |event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();

                let mut visible = window_visible.lock().unwrap();
                *visible = false;

                let item_handle = event.window().app_handle().tray_handle().get_item("hide");
                item_handle.set_title("Open Amie").unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
