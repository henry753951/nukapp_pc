use chrono::{ DateTime, Datelike, Local };
use reqwest;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub mod auth;
pub mod course;
pub mod data;
pub mod score;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    name: String,
    student_id: String,
    department: String,
    admission_year: String,
}

pub struct User {
    client: reqwest::blocking::Client,
    username: String,
    password: String,
    user_data: Option<UserData>,
    last_login: HashMap<String, Option<DateTime<Local>>>
}

impl User {
    pub fn new(username: String, password: String, client: reqwest::blocking::Client) -> Self {
        Self {
            username,
            password,
            client,
            user_data: None,
            last_login: HashMap::new(),
        }
    }
}
