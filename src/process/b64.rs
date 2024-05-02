use crate::cli::base64::Base64Format;
use crate::utils;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = utils::get_reader(input)?;
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let result = match format {
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    Ok(result)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = utils::get_reader(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let data = data.trim();
    let result = match format {
        Base64Format::Standard => STANDARD.decode(data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(data)?,
    };
    Ok(result)
}
