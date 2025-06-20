use std::fs::File;
use std::io::BufReader;
use log::info;
use serde::Deserialize;
use crate::config::EvenflowConfig;
use crate::error::{Error, ResultWrapErr};

#[derive(Deserialize)]
struct Edge {
    #[serde(rename = "n1.id")]
    node1: String,
    #[serde(rename = "n2.id")]
    node2: String,
    #[serde(rename = "type(e)")]
    edge_type: String,
    #[serde(rename = "e.weight")]
    weight: f64,
}

pub(crate) fn run(config: EvenflowConfig) -> Result<(), Error> {
    info!("Calculating evenflow embeddings using edge file {}", config.edge_file.display());
    let mut reader =
        csv::Reader::from_reader(
            BufReader::new(File::open(&config.edge_file)
                .wrap_err(config.edge_file.display().to_string())?)
        );
    for edge in reader.deserialize() {
        let edge: Edge = edge?;
    }
    Ok(())
}