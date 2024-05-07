use crate::{process_jwt_sign, process_jwt_verify, CmdExecutor};

use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "sign jwt")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "verify jwt token")]
    Verify(JwtVerifyOpts),
}

//rcli jwt sign --sub acme --aud device1 --exp 14d
#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long, help = "jwt sub")]
    pub sub: String,
    #[arg(long, help = "jwt aud", default_value = "")]
    pub aud: String,
    #[arg(long, help = "jwt exp")]
    pub exp: String,
    #[arg(long, help = "jwt iss", default_value = "")]
    pub iss: String,
}

// rcli jwt verify -t <token-value>
#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let token = process_jwt_sign(&self.sub, &self.aud, &self.exp, &self.iss)?;
        println!("token: {:?}", token);
        Ok(())
    }
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let data = process_jwt_verify(&self.token)?;
        println!("{:?}", data);
        Ok(())
    }
}
