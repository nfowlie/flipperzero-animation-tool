use tauri::{
    menu::{MenuBuilder, SubmenuBuilder},
    Emitter,
};
use tauri_plugin_dialog::DialogExt;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("flipzero_firmware_dir", "Change Firmware Directory")
                .text("quit", "Quit")
                .build()?;

            let help_menu = SubmenuBuilder::new(app, "Help")
                .text("check_update", "Check For Updates")
                .text("about", "About")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &help_menu])
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
                            "show_firmware_modal",
                            Payload {
                                message: "Show Modal".into(),
                            },
                        )
                        .unwrap(),
                    "check_update" => app_handle
                        .emit(
                            "show_update_modal",
                            Payload {
                                message: app_handle.package_info().version.to_string(),
                            },
                        )
                        .unwrap(),
                    "about" => app_handle
                        .emit(
                            "show_about_modal",
                            Payload {
                                message: app_handle.package_info().version.to_string(),
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
