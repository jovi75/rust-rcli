pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
};
use clap::Parser;
use std::path::Path;

pub mod base64;
pub mod csv;
pub mod genpass;

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
}

fn verify_input_file(filename: &str) -> anyhow::Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("test"), Err("file not exist"));
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("file not exist"));
        assert_eq!(verify_input_file("README.md"), Ok("README.md".into()));
    }
}
