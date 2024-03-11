use chrono::{ Datelike, Local };
use reqwest;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use serde_json::Value;
use std::error::Error;

use crate::utils::decode_big5_to_utf8;
use super::User;

#[derive(Debug, Deserialize, Serialize)]
struct Course {
    課號: String,
    課程名稱: String,
    學分數: String,
    修別: String,
    期中成績: String,
    學期成績: String,
    備註: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SchoolYear {
    學期: String,
    修習學分數: String,
    實得學分數: String,
    平均成績: String,
    排名: String,
    課程: Vec<Course>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    歷年總修習學分數: String,
    歷年實得學分數: String,
    歷年平均成績: String,
    歷年全班排名: String,
    節至學期: String,
    各學期: Vec<SchoolYear>,
}

impl User {
    pub fn get_scores(&mut self) {
        let mut data = Data {
            歷年總修習學分數: "".to_string(),
            歷年實得學分數: "".to_string(),
            歷年平均成績: "".to_string(),
            歷年全班排名: "".to_string(),
            節至學期: "".to_string(),
            各學期: Vec::new(),
        };

        // https://aca.nuk.edu.tw/Student2/SO/ScoreQuery.asp
        let res = self.client
            .get("https://aca.nuk.edu.tw/Student2/SO/ScoreQuery.asp")
            .send()
            .unwrap();
        let html = decode_big5_to_utf8(res).unwrap();
        let document = Html::parse_document(&html);

        // 歷年總成績
        let selector = Selector::parse(r#"table[border="0"][style*="color: #800080"]"#).unwrap();
        let bottom_table = document.select(&selector).last().unwrap();
        for (i, cell) in bottom_table.select(&Selector::parse("td").unwrap()).enumerate() {
            match i {
                1 => {
                    data.歷年總修習學分數 = cell.text().collect::<String>();
                }
                2 => {
                    data.歷年實得學分數 = cell.text().collect::<String>();
                }
                3 => {
                    data.歷年平均成績 = cell.text().collect::<String>();
                }
                4 => {
                    data.歷年全班排名 = cell.text().collect::<String>();
                }
                _ => (),
            }
        }
        // 各學期名稱
        let selector = Selector::parse(
            r#"font[face="標楷體"][size="3"][color="\#0000FF"]"#
        ).unwrap();
        let semester_names = document
            .select(&selector)
            .map(|e| e.text().collect::<String>())
            .collect::<Vec<String>>();

        data.各學期 = semester_names
            .iter()
            .map(|semester| {
                let mut school_year = SchoolYear {
                    學期: semester.to_string(),
                    修習學分數: "".to_string(),
                    實得學分數: "".to_string(),
                    平均成績: "".to_string(),
                    排名: "".to_string(),
                    課程: Vec::new(),
                };
                school_year
            })
            .collect::<Vec<SchoolYear>>();

        // 各學期排名等 table border="0" style*="color: #800000"
        let selector = Selector::parse(r#"table[border="0"][style*="color: #800000"]"#).unwrap();
        let summary_tables = document.select(&selector);
        for (s_index, table) in summary_tables.enumerate() {
            if table.text().collect::<String>().contains("學習成效期中預警") {
                continue;
            }
            for (index, cell) in table.select(&Selector::parse("td").unwrap()).enumerate() {
                let _text = cell.text().collect::<String>();
                let _text = _text.trim();
                // let key = _text.split("：").collect::<Vec<&str>>()[0];
                let value = _text.split("：").collect::<Vec<&str>>()[1];
                // println!("{}: {}", key, value);
                match index {
                    0 => {
                        data.各學期[s_index].修習學分數 = value.to_string();
                    }
                    1 => {
                        data.各學期[s_index].實得學分數 = value.to_string();
                    }
                    2 => {
                        data.各學期[s_index].平均成績 = value.to_string();
                    }
                    3 => {
                        data.各學期[s_index].排名 = value.to_string();
                    }
                    _ => (),
                }
            }
        }

        // 成績表格
        let selector = Selector::parse("table[border=\"1\"]").unwrap();
        let scores_tables = document.select(&selector);
        for (index, table) in scores_tables.enumerate() {
            let score_data = crate::utils::table2json(table);
            let courses: Vec<Course> = score_data
                .into_iter()
                .map(|course| Course {
                    課號: course.get("課號").unwrap().to_string(),
                    課程名稱: course.get("課程名稱").unwrap().to_string(),
                    學分數: course.get("學分數").unwrap().to_string(),
                    修別: course.get("修別").unwrap().to_string(),
                    期中成績: course.get("期中成績").unwrap().to_string(),
                    學期成績: course.get("學期成績").unwrap().to_string(),
                    備註: course.get("備註").unwrap().to_string(),
                })
                .collect();
            data.各學期[index].課程 = courses;
        }

        println!("{}", serde_json::to_string(&data).unwrap());
    }
}
