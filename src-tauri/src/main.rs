// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use nuk::user::User;
use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use tauri::{ api::path::{ app_data_dir, config_dir }, App, Manager, AppHandle };
use std::{ sync::Mutex };
// mods
mod config_reader;
mod setup;
mod utils;

mod commands;
mod nuk {
    pub mod course;
    pub mod user;
}

pub struct Storage {
    user: Mutex<Option<User>>,
}

#[tauri::command]
fn open_devtools(app_handle: AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
        window.open_devtools();
    }
}

fn main() {
    println!("Hello, NUK2");
    tauri::Builder
        ::default()
        .manage(Storage {
            user: Mutex::new(None),
        })
        .setup(setup::init)
        .invoke_handler(
            tauri::generate_handler![
                open_devtools,
                commands::login::login,
                commands::logout::logout,
                commands::get_all_course::get_all_course,
                commands::get_user_score::get_user_score
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
