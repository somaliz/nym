// Copyright 2020-2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use clap::{crate_name, crate_version, Parser};
use colored::Colorize;
use lazy_static::lazy_static;
use log::error;
use nym_bin_common::build_information::BinaryBuildInformation;
use nym_bin_common::logging::{maybe_print_banner, setup_logging};
use nym_bin_common::output_format::OutputFormat;
use nym_network_defaults::setup_env;
use std::error::Error;

mod commands;
mod config;
pub(crate) mod error;
mod node;
pub(crate) mod support;

lazy_static! {
    pub static ref PRETTY_BUILD_INFORMATION: String =
        BinaryBuildInformation::new(env!("CARGO_PKG_VERSION")).pretty_print();
}

// Helper for passing LONG_VERSION to clap
fn pretty_build_info_static() -> &'static str {
    &PRETTY_BUILD_INFORMATION
}

#[derive(Parser)]
#[clap(author = "Nymtech", version, about, long_version = pretty_build_info_static())]
struct Cli {
    /// Path pointing to an env file that configures the gateway.
    #[clap(short, long)]
    pub(crate) config_env_file: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    command: commands::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    setup_logging();
    maybe_print_banner(crate_name!(), crate_version!());

    let args = Cli::parse();
    setup_env(args.config_env_file.as_ref());

    commands::execute(args).await.map_err(|err| {
        if atty::is(atty::Stream::Stdout) {
            let error_message = format!("{err}").red();
            error!("{error_message}");
            error!("Exiting...");
        }
        err
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
