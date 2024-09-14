use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ App, Manager, AppHandle, State };
use serde_json::Value;

use crate::{ nuk, Storage };

#[tauri::command]
pub fn get_user_data(storage: State<Storage>) -> Value {
    let mut binding = storage.user.lock().unwrap();
    let user = binding.as_mut().unwrap();
    user.auth("教務系統".to_string());
    user.get_data().unwrap()
}

#[test]
fn test_get_user_data() {
    dotenv().ok();
    let username = env::var("NUK_USERNAME").expect("USERNAME not found");
    let password = env::var("NUK_PASSWORD").expect("PASSWORD not found");
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let mut user = nuk::user::User::new(username, password, client);
    user.auth("教務系統".to_string());
    user.get_data();
}
