use tauri_plugin_store::StoreExt;

use crate::prelude::*;

#[tauri::command]
pub async fn greet(name: String, state: AppStateType<'_>) -> Result<String, String> {
    let mut state = state.lock().await;
    state.counter += 1;
    Ok(format!(
        "Hello, {}! You've been greeted from Rust {} times!",
        name, state.counter
    ))
}

#[tauri::command]
pub async fn get_store_meta<R: Runtime>(state: AppHandle<R>) -> Result<(), String> {
    let store = state.get_store("store.json").unwrap();

    if store.has("meta") {}
    Ok(())
}
