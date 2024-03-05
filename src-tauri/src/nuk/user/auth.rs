use chrono::{ Datelike, Local };
use reqwest;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::io::{self, BufReader, Read};
use reqwest::Response;
use encoding_rs::BIG5;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::utils::decode_big5_to_utf8;
use crate::nuk::user::User;


impl User {
    pub async fn auth(&self) {
        let login_url = "https://aca.nuk.edu.tw/Student2/login.asp";
        let post_url = "https://aca.nuk.edu.tw/Student2/Menu1.asp";
        println!("❤️ 學號: {}", self.username);
        println!("🤩 登入中...");
        // 實作 auth 方法
        let client = reqwest::Client::new();
        let res = client.get(login_url).send().await.unwrap();
        // BIG5 to UTF-8
        let html = decode_big5_to_utf8(res).await.unwrap();
        let document = scraper::Html::parse_document(&html);
        // 取得所有input成為form_data
        // key : name
        // value : value
        let selector = Selector::parse("input").unwrap();
        let mut inputs = json!({});
        for input in document.select(&selector) {
            let name = input.value().attr("name").unwrap();
            let value = input.value().attr("value").unwrap();
            inputs[name] = value.into();
        }
        // test
        println!("{:?}", inputs);
    }
}
