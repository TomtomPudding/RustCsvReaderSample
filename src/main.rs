use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "State")]
    state: String,
}

// https://qiita.com/algebroid/items/c456d4ec555ae04c7f92#%E3%83%87%E3%83%AA%E3%83%9F%E3%82%BF%E3%82%AF%E3%82%A9%E3%83%BC%E3%83%88%E3%81%9D%E3%81%97%E3%81%A6%E5%8F%AF%E5%A4%89%E9%95%B7%E3%83%AC%E3%82%B3%E3%83%BC%E3%83%89
fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("start read csv");
    for result in BufReader::new(File::open("csv/test.csv")?).lines() {
        let ll = result?;
        println!("{}", ll);
    }
    println!("end read csv");

    let mut rdr = csv::ReaderBuilder::new()
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(File::open("csv/test.csv")?);
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    println!("end read csv");
    Ok(())
}
