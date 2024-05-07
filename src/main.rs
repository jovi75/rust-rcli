// cli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use cli::{cli::Opts, CmdExecutor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    tracing_subscriber::fmt::init();
    opt.cmd.execute().await?;
    Ok(())
}
