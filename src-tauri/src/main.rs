// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
// use tauri::{WindowBuilder}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let flipzero_firmware_dir = CustomMenuItem::new(
        "flipzero_firmware_dir".to_string(),
        "Change Firmware Directory",
    );
    let submenu = Submenu::new(
        "File",
        Menu::new().add_item(flipzero_firmware_dir).add_item(quit),
    );
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(submenu);

    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet])
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "flipzero_firmware_dir" => {
                let window = event.window();
                window
                    .emit(
                        "my_event",
                        Payload {
                            message: "Test Message".into(),
                        },
                    )
                    .unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
