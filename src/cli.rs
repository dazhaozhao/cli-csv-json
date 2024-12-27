pub mod opts;
use opts::CsvOpts;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv or convert csv to other formats")]
    CSV(CsvOpts),
}