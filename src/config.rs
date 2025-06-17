use std::path::PathBuf;
use clap::{command, crate_description, crate_version, Arg, Command};
use crate::error::Error;

pub(crate) enum Config {
    Evenflow(EvenflowConfig),
}

pub(crate) struct EvenflowConfig {
    pub edge_file: PathBuf,
}

mod cmds {
    pub(crate) const EVENFLOW: &str = "evenflow";
    pub(crate) const EVENFLOW_ABOUT: &str = "Calculate evenflow embeddings.";
}

mod args {
    pub(crate) const EDGE_FILE: &str = "edge-file";
    pub(crate) const EDGE_FILE_SHORT: char = 'e';
    pub(crate) const EDGE_FILE_HELP: &str = "Input file with edges";
}

pub(crate) fn get_config() -> Result<Config, Error> {
    let matches =
        command!()
            .subcommand(
                Command::new(cmds::EVENFLOW).about(cmds::EVENFLOW_ABOUT).arg(
                    Arg::new(args::EDGE_FILE).short(args::EDGE_FILE_SHORT)
                        .help(args::EDGE_FILE_HELP)
                        .value_parser(clap::value_parser!(PathBuf))
                )
            )
            .version(crate_version!())
            .about(crate_description!())
            .get_matches();
    match matches.subcommand() {
        Some((subcommand, submatches)) => {
            match subcommand {
                cmds::EVENFLOW => {
                    let edge_file = submatches.get_one::<PathBuf>(args::EDGE_FILE)
                        .cloned()
                        .ok_or_else(|| Error::from("Edge file is required for evenflow."))?;
                    let config = EvenflowConfig { edge_file };
                    Ok(Config::Evenflow(config))
                },
                _ => {
                    Err(Error::from(format!("Unknown command: {}. {}", subcommand,
                                                   known_subcommands_message())))
                }
            }
        }
        None => {
            Err(Error::from(format!("No subcommand provided. {}",
                                    known_subcommands_message())))
        }
    }
}

fn known_subcommands_message() -> &'static str {
    "Only known subcommand is 'evenflow'."
}