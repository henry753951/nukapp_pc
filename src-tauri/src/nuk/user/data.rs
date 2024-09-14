use scraper::{ Html, Selector };
use serde::de;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::error::Error;

use crate::utils::decode_big5_to_utf8;
use super::User;
use super::UserData;

impl User {
    pub fn get_data(&mut self)  -> Result<Value, Box<dyn Error>>  {
        let url = "https://aca.nuk.edu.tw/Graduate/GraduateDetail/Menu.asp";
        let res = self.client.get(url).send().unwrap();
        let html = decode_big5_to_utf8(res).unwrap();
        let document = Html::parse_document(&html);
        let selector = Selector::parse("font").unwrap();
        let fonts = document.select(&selector);

        for font in fonts {
            let str = font.text().collect::<Vec<_>>().join("");
            if str.contains("姓名：") {
                let cols = str.split("　　　　　　").collect::<Vec<_>>();
                let student_id = cols[0].split("：").last().unwrap().to_string();
                let name = cols[1].split("：").last().unwrap().to_string();
                let department = cols[2].split("：").last().unwrap().to_string();
                let admission_year = cols[3].split("：").last().unwrap().to_string();
                println!("學號：{}", student_id);
                println!("姓名：{}", name);
                println!("系所：{}", department);
                println!("入學學年度：{}", admission_year);

                self.user_data = Some(UserData {
                    name,
                    student_id,
                    department,
                    admission_year,
                });
                break;
            }
        }
        Ok(json!(self.user_data))
    }
}
