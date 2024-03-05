use tokio::io::AsyncReadExt;
use encoding_rs_io::DecodeReaderBytesBuilder;
use encoding_rs::BIG5;
use reqwest::Response;
use std::io::{self, BufReader, Read};


pub async fn decode_big5_to_utf8(res: Response) -> io::Result<String> {
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(BIG5))
        .build(io::Cursor::new(res.bytes().await.unwrap()));
    let mut reader = BufReader::new(decoder);
    let mut html = String::new();
    reader.read_to_string(&mut html)?;
    Ok(html)
}