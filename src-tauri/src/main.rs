use tauri::{menu::MenuBuilder, Emitter};
use tauri_plugin_dialog::DialogExt;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("quit", "Quit")
                .text("flipzero_firmware_dir", "Change Firmware Directory")
                .build()?;

            app.set_menu(menu)?;
            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                println!("menu event: {:?}", event.id());

                match event.id().0.as_str() {
                    "quit" => {
                        app_handle.exit(0);
                    }

                    "flipzero_firmware_dir" => app_handle
                        .emit(
                            "show_modal",
                            Payload {
                                message: "Show Modal".into(),
                            },
                        )
                        .unwrap(),
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!());
}
