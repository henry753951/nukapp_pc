use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ api::path::{ app_data_dir, config_dir }, App, AppHandle, Manager, State };
use crate::{ nuk, Storage };

#[tauri::command]
pub fn login(username: String, password: String, storage: State<Storage>) -> Value {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let mut user = nuk::user::User::new(username, password, client);
    user.auth("教務系統".to_string());
    let data = user.get_data().unwrap();
    let mut binding = storage.user.lock().unwrap();
    *binding = Some(user);
    data
}
#[test]
fn test_login() {
    dotenv().ok();
    let username = env::var("NUK_USERNAME").expect("USERNAME not found");
    let password = env::var("NUK_PASSWORD").expect("PASSWORD not found");
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let mut user = nuk::user::User::new(username, password, client);
    user.auth("教務系統".to_string());
    user.auth("選課系統".to_string());
}
