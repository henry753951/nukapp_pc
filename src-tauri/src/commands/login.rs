use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use dotenv::dotenv;
use std::env;
use tauri::{ api::path::{ app_data_dir, config_dir }, App, Manager, AppHandle };
use crate::nuk;

#[tauri::command]
pub async fn login(username: String, password: String)  {
    let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let user = nuk::user::User::new(username, password, client);
    user.auth().await;
}
#[tokio::test]
async fn test_login() {
    dotenv().ok();
    let username = env::var("NUK_USERNAME").expect("USERNAME not found");
    let password = env::var("NUK_PASSWORD").expect("PASSWORD not found");
    let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let user = nuk::user::User::new(username, password, client);
    user.auth().await;
}
