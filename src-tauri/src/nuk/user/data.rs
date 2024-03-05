use chrono::{Datelike, Local};
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;

use crate::nuk::user::User;
impl User {
    pub fn get_data(&self) {
        // 實作 get_user_data 方法
    }
}