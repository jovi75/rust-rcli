use crate::{process_decode, process_encode, CmdExecutor};

use super::verify_file;
use anyhow::Result;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encodes a string to Base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decodes a string from Base64")]
    Decode(Base64DecodeOpts),
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> Result<()> {
        let result = process_encode(&self.input, self.format)?;
        println!("{}", result);
        Ok(())
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> Result<()> {
        let result = process_decode(&self.input, self.format)?;
        let decoded = String::from_utf8(result)?;
        println!("{}", decoded);
        Ok(())
    }
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "encode", about = "Encodes a string to Base64")]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid base64 format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
