#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/op-rs/kona/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod cli;
pub mod flag;

use clap::Parser;
use cli::Cli;
use anyhow::Result;
use kona_supervisor_service::{SupervisorService, SupervisorServiceConfig};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Starting op-supervisor with config:\n{:#?}", cli);
    
    // Initialize supervisor service here using `cli.global` and `cli.metrics`
    let config = SupervisorServiceConfig {
        // Example address, make this configurable
        rpc_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000),
    };
    let service = SupervisorService::new(config)?;
    let server_handle = service.run().await?;

    println!("Supervisor service running. Press Ctrl+C to shut down.");

    // Wait for shutdown signal (Ctrl+C)
    tokio::signal::ctrl_c().await?;
    println!("Shutdown signal received.");

    // Optional: Graceful shutdown logic for the server handle if needed
    // server_handle.stop()?;
    // server_handle.stopped().await;

    println!("Supervisor shut down gracefully.");
    Ok(())
}