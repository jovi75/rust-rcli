// cli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use cli::opts::{Opts, SubCommand};
use cli::process;

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        SubCommand::Csv(opts) => {
            process::process_csv(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
