#![feature(try_trait_v2, never_type)]
use std::{
    convert::Infallible,
    io::BufReader,
    ops::{ControlFlow, FromResidual},
    time::{Duration, Instant},
};
pub use std::{convert::TryInto, error::Error, fs::File, io::Read, ops::Try, process::exit};

use clap::Clap;

pub use bitvec::prelude::*;
pub use image::GenericImageView;
pub use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;

mod decode;
mod encode;
mod utils;
pub use utils::*;

#[derive(Clap)]
struct Opts {
    input: String,
    output: String,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    EncodeText { text: String },
    DecodeText,
    EncodeFile { path: String },
    DecodeFile,
}

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
    FmtSubscriber::builder()
        .try_init()
        .map_err::<Box<dyn Error>, _>(|b| b)?;

    let mut i = Instant::now();

    let opts = Opts::parse();
    info!("OK - parsed opts in {:?}", i.elapsed_now());

    match opts.subcommand {
        Subcommand::EncodeText { text } => encode::bytes(
            &opts.input,
            &opts.output,
            text.bytes().map(Byte),
            text.len(),
        )?,
        Subcommand::EncodeFile { path } => {
            let file = File::open(path)?;
            let len = file.metadata()?.len() as usize;
            encode::bytes(&opts.input, &opts.output, BufReader::new(file).bytes(), len)?
        }
        Subcommand::DecodeText => decode::text(&opts.input, &opts.output, decode::DecodeType::Text)?,
        Subcommand::DecodeFile => decode::text(&opts.input, &opts.output, decode::DecodeType::File)?,
    }
    info!("OK - executed subcmd in {:?}", i.elapsed_now());

    Ok(())
}
