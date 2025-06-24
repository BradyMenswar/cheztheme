use cheztheme_core::themes::{Palette, ThemeType};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;

#[derive(serde::Serialize, Clone, Debug)]
struct ReactTheme {
    id: String,
    name: String,
    #[serde(rename = "type")]
    theme_type: String,
    palette: Palette,
}

#[derive(serde::Serialize, Clone, Debug)]
struct ReactConfig {
    theme_name: String,
    theme: Palette,
}

#[tauri::command]
fn get_theme_names() -> Result<Vec<ReactTheme>, String> {
    let mut react_themes = Vec::new();
    let themes = cheztheme_core::all_theme_names(".local/share/chezmoi/themes").expect("");
    for theme in themes {
        match theme {
            ThemeType::Preset(name) => {
                let theme_palette =
                    cheztheme_core::load_theme(name.as_str(), ".local/share/chezmoi/themes")
                        .expect("")
                        .palette;
                react_themes.push(ReactTheme {
                    id: name.clone() + "-preset",
                    name: name,
                    theme_type: "preset".to_string(),
                    palette: theme_palette,
                });
            }
            ThemeType::Custom(name) => {
                let theme_palette =
                    cheztheme_core::load_theme(name.as_str(), ".local/share/chezmoi/themes")
                        .expect("")
                        .palette;
                react_themes.push(ReactTheme {
                    id: name.clone() + "-custom",
                    name: name,
                    theme_type: "custom".to_string(),
                    palette: theme_palette,
                });
            }
        }
    }
    Ok(react_themes)
}

#[tauri::command]
fn read_config() -> Result<ReactConfig, String> {
    let (doc, _path) = cheztheme_core::config::load_config().expect("");
    let data = &doc["data"]["cheztheme"];
    Ok(ReactConfig {
        theme_name: data["themeName"].as_str().expect("").to_string(),
        theme: Palette {
            base00: data["base00"].as_str().expect("").to_string(),
            base01: data["base01"].as_str().expect("").to_string(),
            base02: data["base02"].as_str().expect("").to_string(),
            base03: data["base03"].as_str().expect("").to_string(),
            base04: data["base04"].as_str().expect("").to_string(),
            base05: data["base05"].as_str().expect("").to_string(),
            base06: data["base06"].as_str().expect("").to_string(),
            base07: data["base07"].as_str().expect("").to_string(),
            base08: data["base08"].as_str().expect("").to_string(),
            base09: data["base09"].as_str().expect("").to_string(),
            base0A: data["base0A"].as_str().expect("").to_string(),
            base0B: data["base0B"].as_str().expect("").to_string(),
            base0C: data["base0C"].as_str().expect("").to_string(),
            base0D: data["base0D"].as_str().expect("").to_string(),
            base0E: data["base0E"].as_str().expect("").to_string(),
            base0F: data["base0F"].as_str().expect("").to_string(),
        },
    })
}

#[tauri::command]
fn apply_theme(theme_name: &str) -> Result<(), String> {
    let new_theme =
        cheztheme_core::load_theme(theme_name, ".local/share/chezmoi/themes").expect("");
    cheztheme_core::apply_theme(theme_name, new_theme).expect("");

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event({
                let window = window.clone();
                move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        window.hide().unwrap();
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_theme_names,
            read_config,
            apply_theme
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
