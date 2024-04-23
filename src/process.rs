use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
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
    let mut reader = Reader::from_path(input)?;
    let mut result: Vec<Player> = Vec::with_capacity(128);
    for record in reader.deserialize() {
        let player: Player = record?;
        result.push(player);
    }
    let json = serde_json::to_string_pretty(&result)?;
    fs::write(output, json)?;
    Ok(())
}
