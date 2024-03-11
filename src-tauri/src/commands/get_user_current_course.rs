use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ api::path::{ app_data_dir, config_dir }, App, AppHandle, Manager, State };
use serde_json::Value;

use crate::{ nuk, Storage };

#[tauri::command]
pub fn get_user_current_course(storage: State<Storage>) -> Value {
    let mut binding = storage.user.lock().unwrap();
    let user = binding.as_mut().unwrap();
    user.auth("選課系統".to_string());
    user.get_current_course().unwrap()
}

#[test]
fn test_get_user_current_course() {
    dotenv().ok();
    let username = env::var("NUK_USERNAME").expect("USERNAME not found");
    let password = env::var("NUK_PASSWORD").expect("PASSWORD not found");
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let user = nuk::user::User::new(username, password, client);
    user.auth("選課系統".to_string());
    user.get_current_course().unwrap();
}
