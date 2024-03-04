use chrono::{Datelike, Local};
use reqwest;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    updateTime: String,
    course_ls: Vec<Course>, // Replace `Course` with the actual type of your courses
}
#[derive(Serialize, Deserialize, Debug)]
struct Course {
    key: String,
    department: String,
    course_id: String,
    department_code: String,
    grade: i32,
    class_type: String,
    course_name: String,
    syllabus_link: String,
    credits: f32,
    requirement_type: String,
    limit: i32,
    registration_confirmed: i32,
    online_number: i32,
    balance: i32,
    teacher: Vec<String>,
    classroom: String,
    course_time: Vec<(String, Vec<String>)>,
    prerequisites: String,
    notes: String,
}
fn process_element(element: &ElementRef) -> Result<Course, Box<dyn Error>> {
    let cells: Vec<_> = element.select(&Selector::parse("td").unwrap()).collect();
    // Check the number of cells to handle different row structures
    let is_full_row = cells.len() == 23;

    let department = cells[0]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let course_id = cells[1]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let department_code = cells[2]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let grade: i32 = cells[3]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .parse()
        .expect("Failed to parse grade");
    let class_type = cells[4]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let course_name = cells[5]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let syllabus_link = cells[5]
        .select(&Selector::parse("a").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
        .to_string();
    let credits: f32 = cells[6]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .parse()
        .expect("Failed to parse credits");
    let requirement_type = cells[7]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();
    let limit: i32 = cells[8]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .parse()
        .expect("Failed to parse limit");
    let registration_confirmed: i32 = cells[9]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .parse()
        .expect("Failed to parse registration_confirmed");
    let online_number: i32 = match cells[10].text().collect::<Vec<_>>().concat().trim() {
        "-" => 0,
        online_number_str => online_number_str
            .parse()
            .expect("Failed to parse online_number"),
    };

    let balance: i32 = cells[11]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .parse()
        .expect("Failed to parse balance");

    let teacher = cells[12]
        .inner_html()
        .split("<br>")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let teacher = teacher
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    let classroom = cells[13]
        .text()
        .collect::<Vec<_>>()
        .concat()
        .trim()
        .to_string();

    let course_time = if is_full_row {
        let mut times = Vec::new();
        let days = vec!["一", "二", "三", "四", "五", "六", "日"];

        for (i, day) in days.iter().enumerate() {
            let time_data = cells[14 + i]
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string();
            if !time_data.is_empty() {
                let time_slots = time_data
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();
                times.push((day.to_string(), time_slots));
            }
        }
        times
    } else {
        Vec::new()
    };

    let prerequisites = if is_full_row {
        cells[21]
            .text()
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_string()
    } else {
        cells[14]
            .text()
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_string()
    };

    let notes = if is_full_row {
        cells[22]
            .text()
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_string()
    } else {
        cells[15]
            .text()
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_string()
    };

    let course = Course {
        key: format!("{}-{}", department, course_id),
        department,
        course_id,
        department_code,
        grade,
        class_type,
        course_name,
        syllabus_link,
        credits,
        requirement_type,
        limit,
        registration_confirmed,
        online_number,
        balance,
        teacher,
        classroom,
        course_time,
        prerequisites,
        notes,
    };
    return Ok(course);
}

pub async fn fetch_new_courses() -> Result<Value, reqwest::Error> {
    let now = Local::now();
    let year = 112;
    let semester = if now.month() >= 7 { 1 } else { 2 };
    let url = "https://course.nuk.edu.tw/QueryCourse/QueryResult.asp";
    println!("year: {}, semester: {}", year, semester);
    let payload = json!({
        "Condition": format!("<tr><td+width=\"\"33%\"\">開課學年：{}　　開課學期：第{}學期</td><td+width=\"\"33%\"\">開課部別：大學部</td><td+width=\"\"34%\"\">開課系所：無</td></tr><tr><td+width=\"\"33%\"\">開課班級：無</td><td+width=\"\"33%\"\">授課教師：無</td><td+width=\"\"34%\"\">上課時間：無</td></tr>", year, semester),
        "Flag": "1",
        "OpenYear": year.to_string(),
        "Helf": semester.to_string(),
        "Pclass": "A",
    });

    let client = reqwest::Client::new();
    let res = client.post(url).form(&payload).send().await?;

    if res.status().is_success() {
        let body = res.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("tr[align=\"center\"]").unwrap();

        let mut course_ls: Vec<Course> = Vec::new();

        for element in document.select(&selector) {
            let course = match process_element(&element) {
                Ok(course) => course,
                Err(_) => {
                    continue;
                }
            };
            course_ls.push(course);
        }
        let out = Output {
            // 2024/01/24 22:00:00
            updateTime: chrono::offset::Utc::now()
                .with_timezone(&chrono::offset::FixedOffset::east(8 * 3600))
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            course_ls: course_ls,
        };
        let json: Value = serde_json::to_value(&out).unwrap();
        return Ok(json);
    } else {
        println!("Failed to fetch data.");

        Ok(json!({}))
    }
}
