// file: cli.rs

#![allow(dead_code)]

use clap::{Error, Parser};

#[derive(Parser)]
#[clap()]
pub struct Cli {
    pub green: Option<String>,
    pub red: Option<String>,
    pub yellows: Option<Vec<String>>,
}

pub fn parse() -> Cli {
    Cli::parse()
} // end fn parse()

pub fn try_parse() -> Result<Cli, Error> {
    Cli::try_parse()
} // end fn parse()

// cli.rs
