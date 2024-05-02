pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};
use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod text;

#[derive(Debug, Parser)]
#[command(name="cli",version,author,about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show or convert CSV")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(text::TextSubCommand),
    #[command(subcommand)]
    Http(http::HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("path not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("test"), Err("file not exist"));
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("file not exist"));
        assert_eq!(verify_file("README.md"), Ok("README.md".into()));
    }
}
