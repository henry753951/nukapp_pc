use serde_json::Value;
use std::{ fs::{ File }, io::{ Read } };
use tauri::{ api::path::{ app_data_dir, config_dir }, App, Manager, AppHandle };
use crate::nuk;

#[tauri::command]
pub async fn get_all_course(refresh: bool, app_handle: AppHandle) -> Value {
    let config = app_handle.config();
    let cache_path = app_data_dir(&config)
        .unwrap()
        .join("Data")
        .join("all_course.json")
        .to_str()
        .unwrap()
        .to_string();
    if refresh || !std::path::Path::new(&cache_path).exists() {
        match nuk::course::fetch_new_courses().await {
            Ok(result) => {
                let json_data = serde_json::to_string_pretty(&result).unwrap();
                let _ = std::fs::write(&cache_path, json_data.clone());
                return result;
            }
            Err(err) => {
                println!("Failed to fetch data: {}", err);
            }
        }
    }

    let mut file = File::open(&cache_path).expect("Failed to open all_course.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read all_course.json");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse all_course.json");
    return json;
}

#[tokio::test]
async fn test_get_all_course() {
    match nuk::course::fetch_new_courses().await {
        Ok(result) => {
            assert!(result != serde_json::json!({}), "Failed to fetch data");
        }
        Err(err) => {
            assert!(false, "Failed to fetch data: {}", err);
        }
    }
}
