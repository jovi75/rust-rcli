// cli csv -i input.csv -o output.json --header -d ','

use std::fs;

use clap::Parser;
use zxcvbn::zxcvbn;

use cli::cli::base64::Base64SubCommand;
use cli::cli::{Opts, SubCommand};
use cli::process::{process_csv, process_decode, process_encode, process_genpass, process_sign};
use cli::{process_gen_key, process_verify, TextSignFormat, TextSubCommand};

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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("Password: {}", password);
            let estimate = zxcvbn(&password, &[]).unwrap();
            eprintln!("Strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let result = process_encode(&opts.input, opts.format)?;
                println!("{}", result);
            }
            Base64SubCommand::Decode(opts) => {
                let result = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8(result)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{:?}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let result = process_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{:?}", result);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_gen_key(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let fname = &opts.output.join("blake3.txt");
                        fs::write(fname, &keys[0])?;
                        println!("blake3 key saved in {:?}", fname);
                    }
                    TextSignFormat::Ed25519 => {
                        let sk = &opts.output.join("ed25519.sk");
                        fs::write(sk, &keys[0])?;
                        println!("signing key saved in {:?}", sk);

                        let pk = &opts.output.join("ed25519.pk");
                        fs::write(pk, &keys[1])?;
                        println!("verifying key saved in {:?}", pk);
                    }
                }
                //todo
                // cli text encrypt -key "xxx" ==> 加密输出 encode base64
                // cli text decrypt -key "xxx" ==> base64 decode binary - 解密输出
            }
        },
    }

    Ok(())
}
