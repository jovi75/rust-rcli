use crate::{process_decrypt, process_encrypt, process_gen_key, process_sign, process_verify};

use super::*;
use anyhow::{anyhow, Error, Ok};
use clap::Parser;
use core::str;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text")]
    Sign(TextSignOpts),
    #[command(about = "Verify text")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate key")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt text")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt text")]
    Decrypt(TextDecryptOpts),
}

// rcli text encrypt -key"xxx"> 加密并输出 base64
#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

// rcli text decrypt -key"XXX" > base64 > binary> 解密文本
#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
}

// rcli text sign --key key --input input --format blake3
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

// rcli text verify --key key --input input --format blake3 --sig sig
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(long)]
    pub sig: String,
}

// rcli text generate --format blake3 --output path
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(anyhow!("Invalid format: {}", s)),
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Blake3 => write!(f, "blake3"),
            Self::Ed25519 => write!(f, "ed25519"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

fn parse_format(s: &str) -> Result<TextSignFormat, Error> {
    s.parse()
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> Result<()> {
        let signed = process_sign(&self.input, &self.key, self.format)?;
        println!("{:?}", signed);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> Result<()> {
        let result = process_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{:?}", result);
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> Result<()> {
        let keys = process_gen_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let fname = &self.output.join("blake3.txt");
                fs::write(fname, &keys[0]).await?;
                println!("blake3 key saved in {:?}", fname);
            }
            TextSignFormat::Ed25519 => {
                let sk = &self.output.join("ed25519.sk");
                fs::write(sk, &keys[0]).await?;
                println!("signing key saved in {:?}", sk);

                let pk = &self.output.join("ed25519.pk");
                fs::write(pk, &keys[1]).await?;
                println!("verifying key saved in {:?}", pk);
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> Result<()> {
        let result = process_encrypt(&self.input, &self.key)?;
        println!("{:?}", result);
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> Result<()> {
        let result = process_decrypt(&self.input, &self.key)?;
        println!("{:?}", result);
        Ok(())
    }
}

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
            TextSubCommand::Encrypt(opts) => opts.execute().await,
            TextSubCommand::Decrypt(opts) => opts.execute().await,
        }
    }
}
