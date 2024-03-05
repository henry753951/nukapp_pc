use chrono::{Datelike, Local};
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub mod auth;
pub mod course;
pub mod data;


pub struct User {
    client: reqwest::Client,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String,client: reqwest::Client) -> Self {
        Self {
            username,
            password,
            client,
        }
    }
}