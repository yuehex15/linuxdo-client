// Linux.Do 客户端 - 极轻量 Tauri 壳
// 支持 settings.ini 配置文件

use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Manager;

fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .unwrap_or(&PathBuf::from("."))
        .to_path_buf()
}

fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_lowercase();
            let value = line[pos + 1..].trim().to_string();
            map.insert(key, value);
        }
    }
    map
}

fn get_default_url() -> &'static str {
    "https://linux.do/"
}

fn apply_config() -> HashMap<String, String> {
    let config_path = get_exe_dir().join("settings.ini");
    let mut config = HashMap::new();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        config = parse_config(&content);
    }

    let proxy_mode = config.get("proxy_mode").map(|s| s.as_str()).unwrap_or("system");
    let mut browser_args = String::new();

    match proxy_mode {
        "direct" => browser_args.push_str("--no-proxy-server"),
        "custom" => {
            if let Some(proxy) = config.get("custom_proxy") {
                browser_args.push_str(&format!("--proxy-server={}", proxy));
            }
        }
        _ => {}
    }

    if config.get("disable_gpu").map(|s| s == "true").unwrap_or(true) {
        if !browser_args.is_empty() { browser_args.push(' '); }
        browser_args.push_str("--disable-gpu");
    }

    if let Some(heap) = config.get("max_js_heap") {
        if !browser_args.is_empty() { browser_args.push(' '); }
        browser_args.push_str(&format!("--js-flags=--max-old-space-size={}", heap));
    }

    if !browser_args.is_empty() {
        if let Ok(existing) = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS") {
            std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", format!("{} {}", existing, browser_args));
        } else {
            std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", browser_args);
        }
    }

    config
}

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
    let config = apply_config();
    let target_url = config.get("url").map(|s| s.clone()).unwrap_or_else(|| get_default_url().to_string());

    tauri::Builder::default()
        .setup(move |app| {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(url) = tauri::Url::parse(&target_url) {
                    let _ = window.navigate(url);
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![toggle_fullscreen, navigate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}