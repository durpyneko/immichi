#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|_window, e| match e {
            // https://github.com/tauri-apps/tauri/issues/6322
            tauri::WindowEvent::Resized(_) => {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
            _ => {}
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
