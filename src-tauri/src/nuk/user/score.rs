use chrono::{Datelike, Local};
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;


impl User {
    pub fn get_scores(&self) {
        // 實作 get_scores 方法
    }
}