use chrono::{ Datelike, Local };
use std::sync::{ Arc, Mutex };
use reqwest;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::io::{ self, BufReader, Read };
use reqwest::{ Client, Response };
use encoding_rs::BIG5;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::utils::decode_big5_to_utf8;
use crate::nuk::user::User;

impl User {
    pub async fn auth(&self) -> bool {
        let login_url = "https://aca.nuk.edu.tw/Student2/login.asp";
        let post_url = "https://aca.nuk.edu.tw/Student2/Menu1.asp";
        println!("❤️  學號: {}", self.username);
        // 實作 auth 方法
        let res = self.client.get(login_url).send().await.unwrap();
        let html = res.text().await.unwrap();
        // threads safely
        let inputs = tokio::task
            ::spawn_blocking(move || {
                let document = scraper::Html::parse_document(&html);
                // 取得所有input成為form_data
                // key : name
                // value : value
                let selector = Selector::parse("input").unwrap();
                let mut inputs = serde_json::Map::new();
                for input in document.select(&selector) {
                    let name = input.value().attr("name").unwrap();
                    let value = input.value().attr("value").unwrap();
                    inputs.insert(name.to_string(), value.into());
                }
                inputs
            }).await
            .unwrap();

        let csrf_token = inputs["CSRFToken"].as_str().unwrap();
        let account_label = inputs.keys().nth(2).unwrap();
        let password_label = inputs.keys().nth(4).unwrap();

        let payload =
            json!({
                "CSRFToken": csrf_token,
                account_label: self.username,
                password_label: self.password,

            });
        println!("Payload: {:?}", payload);
        println!("🔥 登入中...");

        let res = self.client.post(post_url).form(&payload).send().await.unwrap();
        let html = decode_big5_to_utf8(res).await.unwrap();
        let document = scraper::Html::parse_document(&html);

        // if "登出系統" in html
        // 登入成功
        let selector = Selector::parse("a").unwrap();
        let mut is_login = false;
        for a in document.select(&selector) {
            if a.text().collect::<String>() == "登出系統" {
                is_login = true;
            }
        }
        if is_login {
            println!("🎉 登入成功");
            return true;
        } else {
            println!("🚨 登入失敗");
            return false;
        }
    }
}
