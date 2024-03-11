use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;

use crate::utils::decode_big5_to_utf8;
use super::User;
use super::UserData;

impl User {
    pub fn auth(&self, _type: String) -> bool {
        let login_url = match _type.as_str() {
            "選課系統" => "https://course.nuk.edu.tw/Sel/login.asp",
            "教務系統" => "https://aca.nuk.edu.tw/Student2/login.asp",
            _ => panic!("Type not found"),
        };
        let post_url = match _type.as_str() {
            "選課系統" => "https://course.nuk.edu.tw/Sel/SelectMain1.asp",
            "教務系統" => "https://aca.nuk.edu.tw/Student2/Menu1.asp",
            _ => panic!("Type not found"),
        };
        println!("❤️  學號: {}", self.username);

        let res = self.client.get(login_url).send().unwrap();
        let html = res.text().unwrap();

        let inputs = {
            let document = scraper::Html::parse_document(&html);
            // 取得所有input成為form_data
            // key : name
            // value : value
            let selector = Selector::parse("input").unwrap();
            let mut inputs = serde_json::Map::new();
            for input in document.select(&selector) {
                let name = input.value().attr("name");
                match name {
                    None => continue,
                    Some(name) => {
                        let value = input.value().attr("value").unwrap();
                        inputs.insert(name.to_string(), value.into());
                    }
                }
            }
            inputs
        };

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

        let res = self.client.post(post_url).form(&payload).send().unwrap();
        let html = decode_big5_to_utf8(res).unwrap();
        // println!("html: {}", html);
        let document = scraper::Html::parse_document(&html);

        // if "登出系統" in html
        // 登入成功
        let selector = Selector::parse("a").unwrap();
        let mut is_login = false;
        for a in document.select(&selector) {
            let a_text = a.text().collect::<String>();
            if a_text == "登出系統" || a_text == "登出選課系統" {
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
