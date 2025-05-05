//! Contains the supervisor CLI.

use clap::Parser;
use crate::flag::{GlobalArgs, MetricsArgs};

/// CLI for the Rust implementation of the OP Supervisor.
#[derive(Parser, Debug)]
#[command(name = "op-supervisor", about = "Rust implementation of the OP Supervisor")]
pub struct Cli {
    /// Global args
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Prometheus metrics args
    #[command(flatten)]
    pub metrics: MetricsArgs,
}
