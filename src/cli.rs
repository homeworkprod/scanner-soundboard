/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Specify configuration filename (e.g. `config.toml`)
    #[clap(short = 'c', long = "config")]
    pub config_filename: PathBuf,

    /// Specify input device (e.g. `/dev/input/event23`)
    #[clap(short = 'i', long = "input-device")]
    pub input_device: String,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
