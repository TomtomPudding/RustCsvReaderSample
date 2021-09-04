use std::fs::File;
use serde::{Deserialize};
use std::time::Instant;

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

fn read_csv() -> Vec<Record> {
    let file = match File::open("csv/test.csv") {
        Err(why) => panic!("couldn't open csv: {:?}", why),
        Ok(file) => file,
    };
    let mut rdr = csv::ReaderBuilder::new()
        .double_quote(false)
        .escape(Some(b'$'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(file);
    let mut recode_vec: Vec<Record> = Vec::new();
    for result in rdr.deserialize() {
        match result {
            Err(why) => {
                println!("{:?}", why)
            },
            Ok(result) => recode_vec.push(result)
        }
    }
    println!("end read csv");

    // println!("{:?}", recode_vec);
    recode_vec
}

fn main() {
    let start = Instant::now();
    read_csv();
    let duration = start.elapsed();
    println!("Time elapsed in read_csv() is: {:?}", duration);
}