use super::cli::opts::CsvOpts;
use super::cli::{Opts, SubCommand};
use clap::Parser;
use csv::Reader;
use std::collections::HashMap;
use std::fs;
pub struct CsvTransformer;

impl CsvTransformer {
    pub fn transform() -> anyhow::Result<()> {
        let opts = Opts::parse();
        println!("{opts:#?}");
        match opts.cmd {
            SubCommand::CSV(opts) => process_csv(opts)?,
        }
        Ok(())
    }
}

fn process_csv(opts: CsvOpts) -> Result<(), anyhow::Error> {
    let mut reader = Reader::from_path(opts.input)?;
    let records = reader
        .deserialize()
        .map(|result| result.unwrap())
        .collect::<Vec<HashMap<String, serde_json::Value>>>();
    if opts.pretty {
        fs::write(opts.output, serde_json::to_string_pretty(&records)?)?;
    } else {
        fs::write(opts.output, serde_json::to_string(&records)?)?;
    }
    Ok(())
}
