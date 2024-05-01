use anyhow::Error;
use std::fs::File;
use std::io::Read;

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>, Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
