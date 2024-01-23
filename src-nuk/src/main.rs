use reqwest;
use scraper::{ Html, Selector };
use serde::{ Serialize, Deserialize };
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use chrono::{ Local, Datelike };

#[derive(Serialize, Deserialize, Debug)]
struct Course {
    department: String,
    course_id: String,
    department_code: String,
    grade: String,
    class_type: String,
    course_name: String,
    syllabus_link: String,
    credits: String,
    requirement_type: String,
    limit: String,
    registration_confirmed: String,
    online_number: String,
    balance: String,
    teacher: String,
    classroom: String,
    course_time: Vec<(String, Vec<String>)>,
    prerequisites: String,
    notes: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let now = Local::now();
    let year = 112;
    let semester = if now.month() >= 7 { 1 } else { 2 };
    let url = "https://course.nuk.edu.tw/QueryCourse/QueryResult.asp";
    println!("year: {}, semester: {}", year, semester);
    let payload =
        json!({
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
            let cells: Vec<_> = element.select(&Selector::parse("td").unwrap()).collect();
            // Check the number of cells to handle different row structures
            let is_full_row = cells.len() == 23;

            let department = cells[0].text().collect::<Vec<_>>().concat().trim().to_string();
            let mut course_id = cells[1].text().collect::<Vec<_>>().concat().trim().to_string();
            let department_code = cells[2].text().collect::<Vec<_>>().concat().trim().to_string();
            let grade = cells[3].text().collect::<Vec<_>>().concat().trim().to_string();
            let class_type = cells[4].text().collect::<Vec<_>>().concat().trim().to_string();
            let course_name = cells[5].text().collect::<Vec<_>>().concat().trim().to_string();
            let syllabus_link = cells[5]
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .to_string();
            let credits = cells[6].text().collect::<Vec<_>>().concat().trim().to_string();
            let requirement_type = cells[7].text().collect::<Vec<_>>().concat().trim().to_string();
            let limit = cells[8].text().collect::<Vec<_>>().concat().trim().to_string();
            let registration_confirmed = cells[9]
                .text()
                .collect::<Vec<_>>()
                .concat()
                .trim()
                .to_string();
            let online_number = cells[10].text().collect::<Vec<_>>().concat().trim().to_string();
            let balance = cells[11].text().collect::<Vec<_>>().concat().trim().to_string();
            let teacher = cells[12].text().collect::<Vec<_>>().concat().trim().to_string();
            let classroom = cells[13].text().collect::<Vec<_>>().concat().trim().to_string();

            let course_time = if is_full_row {
                let mut times = Vec::new();
                let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

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
                cells[21].text().collect::<Vec<_>>().concat().trim().to_string()
            } else {
                cells[14].text().collect::<Vec<_>>().concat().trim().to_string()
            };

            let notes = if is_full_row {
                cells[22].text().collect::<Vec<_>>().concat().trim().to_string()
            } else {
                cells[15].text().collect::<Vec<_>>().concat().trim().to_string()
            };

            let course = Course {
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

            course_ls.push(course);
        }

        let json_data = serde_json::to_string_pretty(&course_ls).unwrap();
        fs::write("all_course.json", json_data);
    } else {
        println!("Failed to fetch data.");
    }

    Ok(())
}
