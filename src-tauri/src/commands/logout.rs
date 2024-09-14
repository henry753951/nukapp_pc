use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ App, Manager, AppHandle, State };
use crate::{ nuk, Storage };

#[tauri::command]
pub fn logout(storage: State<Storage>) -> Result<bool, String> {
    let mut binding = storage.user.lock().unwrap();
    *binding = None;
    Ok(true)
}
