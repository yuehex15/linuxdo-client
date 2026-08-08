// Linux.Do 客户端 - 极轻量 Tauri 壳

#[tauri::command]
fn toggle_fullscreen(window: tauri::WebviewWindow) -> Result<(), String> {
    let is_full = window.is_fullscreen().map_err(|e| e.to_string())?;
    window.set_fullscreen(!is_full).map_err(|e| e.to_string())
}

#[tauri::command]
fn navigate(window: tauri::WebviewWindow, url: String) -> Result<(), String> {
    window.navigate(tauri::Url::parse(&url).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toggle_fullscreen, navigate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}