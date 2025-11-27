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
async fn command_name<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    _state: AppStateType<'_>,
) -> Result<(), String> {
    Ok(())
}
