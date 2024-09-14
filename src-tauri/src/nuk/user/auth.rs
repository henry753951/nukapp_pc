use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;

use crate::utils::decode_big5_to_utf8;
use super::User;
use super::UserData;

impl User {
    pub fn auth(&mut self, _type: String) -> bool {
        if self.username == "" || self.password == "" {
            panic!("❌  學號或密碼未設定");
        }
        if let Some(last_login) = self.last_login.get(&_type) {
            if let Some(time) = last_login {
                if time.timestamp() + 600 > chrono::Local::now().timestamp() {
                    let remaining_seconds =
                        time.timestamp() + 600 - chrono::Local::now().timestamp();
                    println!("🔒  登入狀態仍有效 (剩下{}秒)", remaining_seconds);
                    return true;
                }
            }
        }

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
            // 取得所有input成為form_data，key : name，value : value
            let selector = Selector::parse("input").unwrap();
            let mut inputs = Vec::new();
            for input in document.select(&selector) {
                let name = input.value().attr("name");
                match name {
                    None => {
                        continue;
                    }
                    Some(name) => {
                        let style = input.value().attr("style");
                        if name == "CSRFToken" || style != Some("display:none") {
                            if let Some(value) = input.value().attr("value") {
                                inputs.push((name.to_string(), value.to_string()));
                            }
                        }
                    }
                }
            }
            inputs
        };

        for (key, value) in &inputs {
            println!("{}: {}", key, value);
        }

        let csrf_token = inputs
            .iter()
            .find(|(key, _)| key == "CSRFToken")
            .map(|(_, value)| value)
            .unwrap();

        let account_label = &inputs[1].0;

        let password_label = &inputs[2].0;

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

            self.last_login.insert(_type.to_string(), Some(chrono::Local::now()));
            return true;
        } else {
            println!("🚨 登入失敗");
            return false;
        }
    }
}
