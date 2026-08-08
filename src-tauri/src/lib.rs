// Linux.Do 客户端 - 极轻量 Tauri 壳

use tauri::WebviewWindowExt;

#[tauri::command]
fn open_external(url: String) {
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("cmd").args(["/C", "start", "", &url]).spawn(); }
    #[cfg(not(target_os = "windows"))]
    { let _ = std::process::Command::new("xdg-open").arg(&url).spawn(); }
}

#[tauri::command]
fn navigate(window: tauri::Window, action: String) -> Result<(), String> {
    match action.as_str() {
        "back" => { let _ = window.eval("window.history.back()"); }
        "forward" => { let _ = window.eval("window.history.forward()"); }
        "reload" => { let _ = window.eval("window.location.reload()"); }
        "home" => { let _ = window.eval("window.location.href = 'https://linux.do/'"); }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
fn toggle_fullscreen(window: tauri::Window) -> Result<(), String> {
    let is_full = window.is_fullscreen().map_err(|e| e.to_string())?;
    window.set_fullscreen(!is_full).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_external, navigate, toggle_fullscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}