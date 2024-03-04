#![allow(unused_imports)]
use crate::config_reader::ConfigReader;
use platform_dirs::{AppDirs, UserDirs};
use tauri::{App, Manager};
use window_shadows::set_shadow;
use window_vibrancy::{
    apply_acrylic, apply_mica, apply_tabbed, apply_vibrancy, NSVisualEffectMaterial,
};

/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_dirs = AppDirs::new(Some("NUKApp"), true).unwrap();
    let config_file_path = app_dirs.config_dir.join("config-file");
    // create App data directory
    if !config_file_path.exists() {
        std::fs::create_dir_all(&config_file_path)?;
    }

    
    let window = app.get_window("main").unwrap();
    // let config_reader =
    //     ConfigReader::new(config_file_path, "config.example.json".to_string());
    // let config = config_reader.read_config();

    // macOS
    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // Windows
    #[cfg(target_os = "windows")]
    apply_acrylic(&window, Some((100, 100, 100, 55)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    // set shadow
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&window, true).unwrap();

    Ok(())
}
