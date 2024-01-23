// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::Value;
use std::{ fs::{ File, OpenOptions }, io::{ Read, Write } };
// mods
mod config_reader;
mod setup;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    format!("Hello, {}!", name)
}

// return all_course.json to js
#[tauri::command]
fn get_all_course() -> Value {
    let mut file = File::open("all_course.json").expect("Failed to open all_course.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read all_course.json");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse all_course.json");
    return json;
}
fn main() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window::init())
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![greet, get_all_course])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
