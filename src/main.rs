#![feature(min_specialization)]
use std::{error::Error, process::exit};

use clap::Clap;

pub use tracing::{debug, info};
pub use bitvec::prelude::*;
use tracing_subscriber::FmtSubscriber;

mod encode;
mod decode;

#[derive(Clap)]
struct Opts {
    input: String,
    output: String,
    #[clap(subcommand)]
    subcommand: Subcommand
}

#[derive(Clap)]
enum Subcommand {
    EncodeText {
        text: String
    },
    DecodeText,
}

trait PrintError: Error {
    fn print(&self) {
        println!("{}", self)
    }
}

type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    match main_inner() {
        Ok(()) => {}
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}

fn main_inner() -> Result {
    FmtSubscriber::builder().try_init().map_err::<Box<dyn Error>, _>(|b| b)?;
    let opts = Opts::parse();
    match opts.subcommand {
        Subcommand::EncodeText {
            text
        } => encode::text(opts.input, opts.output, text)?,
        Subcommand::DecodeText => decode::text(opts.input, opts.output)?,
    }

    Ok(())
}
