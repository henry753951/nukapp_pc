use tauri::{App, Manager};
use window_vibrancy::{self, NSVisualEffectMaterial};
use crate::config_reader::ConfigReader;

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();

    let config_reader =
        ConfigReader::new("config.json".to_string(), "config.example.json".to_string());
    config_reader.read_config();

    // macOS
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // Windows
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_acrylic(&win, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}
