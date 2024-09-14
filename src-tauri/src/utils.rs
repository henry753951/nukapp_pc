use tokio::io::AsyncReadExt;
use encoding_rs_io::DecodeReaderBytesBuilder;
use encoding_rs::BIG5;
use reqwest::blocking::Response;
use scraper::ElementRef;
use scraper::{ Html, Selector };
use std::collections::HashMap;
use std::io::{ self, BufReader, Read };

pub fn decode_big5_to_utf8(res: Response) -> io::Result<String> {
    let bytes = res.bytes().unwrap();
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(BIG5))
        .build(io::Cursor::new(bytes));
    let mut reader = BufReader::new(decoder);
    let mut html = String::new();
    reader.read_to_string(&mut html)?;
    Ok(html)
}

pub fn table2json(table: ElementRef) -> Vec<HashMap<String, String>> {
    let mut data_vec = vec![];
    let mut headers = vec![];

    // Get headers from the first row
    for cell in table.select(&Selector::parse("tr:first-child th").unwrap()) {
        let data = cell.text().collect::<String>();
        let data = data.split("　").collect::<String>();
        headers.push(data);
    }

    // Get data
    for row in table.select(&Selector::parse("tr:not(:first-child)").unwrap()) {
        let mut course = HashMap::new();
        for (j, cell) in row.select(&Selector::parse("td").unwrap()).enumerate() {
            let data = cell.text().collect::<String>();
            let key = &headers[j];
            course.insert(key.clone(), data);
        }
        data_vec.push(course);
    }
    data_vec
}
