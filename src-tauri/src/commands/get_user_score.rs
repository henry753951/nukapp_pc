use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ api::path::{ app_data_dir, config_dir }, App, AppHandle, Manager, State };
use serde_json::Value;


use crate::{ nuk, Storage };

#[tauri::command]
pub fn get_user_score(storage: State<Storage>) -> Option<nuk::user::score::Data> {
    
    let mut binding = storage.user.lock().unwrap();
    let mut user = binding.as_mut();
    match user {
        Some(user) => {
            user.auth("教務系統".to_string());
            Some(user.get_scores().unwrap())
        },
        None => {
            println!("User not found");
            None
        }
        
    }

}

#[test]
fn test_get_user_score() {
    dotenv().ok();
    let username = env::var("NUK_USERNAME").expect("USERNAME not found");
    let password = env::var("NUK_PASSWORD").expect("PASSWORD not found");
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let mut user = nuk::user::User::new(username, password, client);
    user.auth("教務系統".to_string());
    user.get_scores();
}
