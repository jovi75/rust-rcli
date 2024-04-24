use crate::cli::csv::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut result: Vec<Value> = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let rec = record?;
        let jason_value = headers.iter().zip(rec.iter()).collect::<Value>();
        result.push(jason_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
    };
    fs::write(output, content)?;
    Ok(())
}
