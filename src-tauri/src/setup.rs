#![allow(unused_imports)]
use crate::config_reader::ConfigReader;
use tauri::{App, Manager};
use window_shadows::set_shadow;
use window_vibrancy::{
    apply_acrylic, apply_mica, apply_tabbed, apply_vibrancy, NSVisualEffectMaterial,
};

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();

    let config_reader =
        ConfigReader::new("config.json".to_string(), "config.example.json".to_string());
    let config = config_reader.read_config();

    // macOS
    #[cfg(target_os = "macos")]
    apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // Windows
    #[cfg(target_os = "windows")]
    apply_acrylic(&win, Some((20, 20, 20, 10)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    // apply_tabbed(&win, Some(true)).expect("Failed to apply tabbed window style");

    // set shadow
    let _ = win.set_shadow(true);

    Ok(())
}
