use anyhow::{Error, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::fs::File;
use std::io::Read;

use crate::cli::base64::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let result = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    println!("{}", result);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim();
    let result = match format {
        Base64Format::Standard => STANDARD.decode(data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(data)?,
    };
    println!("{}", String::from_utf8(result)?);
    Ok(())
}
fn get_reader(input: &str) -> Result<Box<dyn Read>, Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
