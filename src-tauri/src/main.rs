// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use tauri::{ api::path::{ app_data_dir, config_dir }, App, Manager, AppHandle };
// mods
mod config_reader;
mod setup;
mod utils;

mod commands;
mod nuk {
    pub mod course;
    pub mod user;
}

#[tauri::command]
fn open_devtools(app_handle: AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.open_devtools();
}



fn main() {
    println!("Hello, NUK2");
    tauri::Builder
        ::default()
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![open_devtools, commands::get_all_course::get_all_course,commands::login::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
