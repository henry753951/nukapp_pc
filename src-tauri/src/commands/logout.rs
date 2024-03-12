use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ api::path::{ app_data_dir, config_dir }, App, AppHandle, Manager, State };
use crate::{ nuk, Storage };

#[tauri::command]
pub fn logout(storage: State<Storage>) -> Result<bool, String> {
    let mut binding = storage.user.lock().unwrap();
    *binding = None;
    Ok(true)
}
