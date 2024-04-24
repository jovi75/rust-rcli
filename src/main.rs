// cli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use cli::opts::{Opts, SubCommand};
use cli::process;

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process::process_csv(&opts.input, output, opts.format)?;
        }
    }

    Ok(())
}
