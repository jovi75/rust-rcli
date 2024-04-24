// cli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use cli::cli::base64::Base64SubCommand;
use cli::cli::{Opts, SubCommand};
use cli::process::{process_csv, process_decode, process_encode, process_genpass};

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
