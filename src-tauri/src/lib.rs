use tauri::{async_runtime::Mutex, Manager};

use serde_json::json;
use tauri_plugin_store::StoreExt;

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(if cfg!(debug_assertions) {
                    tauri_plugin_log::log::LevelFilter::Debug
                } else {
                    tauri_plugin_log::log::LevelFilter::Info
                })
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let store = app.store("store.json")?;
            if !store.has("meta") {
                let value = store.get("meta");
            }
            app.manage(Mutex::new(state::AppState::default()));
            Ok(())
        });

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_store_meta
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub(crate) mod prelude {
    use crate::state::AppState;
    use tauri::{async_runtime::Mutex, State};

    pub use tauri::{AppHandle, Runtime, Window};
    pub type AppStateType<'a> = State<'a, Mutex<AppState>>;
}
