use chrono::{ Datelike, Local };
use reqwest;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use super::User;
use super::UserData;

impl User {
    pub fn get_current_course(&self) -> Result<Value, Box<dyn Error>> {
        // https://course.nuk.edu.tw/Sel/query3.asp
        let res = self.client.get("https://course.nuk.edu.tw/Sel/query3.asp").send().unwrap();
        let html = crate::utils::decode_big5_to_utf8(res).unwrap();
        let document = Html::parse_document(&html);
        let selector = Selector::parse("table").unwrap();
        let mut tables = document.select(&selector);
        let table = tables.next().unwrap();

        Ok(json!(crate::utils::table2json(table)))
    }
}
