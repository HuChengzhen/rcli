use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};
use anyhow::Result;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader: Reader<std::fs::File> = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let player: Player = result?;
        println!("{:?}", player);
        ret.push(player);
    }
    let 
    contents = serde_json::to_string_pretty(&ret)?;
    fs::write(output, contents)?;
    Ok(())
}
