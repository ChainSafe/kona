//! Contains the main Supervisor service runner.

use anyhow::Result;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use kona_supervisor_core::{
  Supervisor,
  SupervisorRpc,
};
use kona_supervisor_rpc::SupervisorApiServer;
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, warn};

/// Configuration for the Supervisor service.
#[derive(Debug, Clone)]
pub struct SupervisorServiceConfig {
    /// The socket address for the RPC server to listen on.
    // TODO:: refactoring required. RPC config should be managed in it's domain
    pub rpc_addr: SocketAddr,
    // Add other configuration fields as needed (e.g., connection details for L1/L2 nodes)
}

/// The main Supervisor service structure.
#[derive(Debug)]
pub struct SupervisorService {
    config: SupervisorServiceConfig,
    supervisor_core: Arc<Supervisor>,
    rpc_impl: SupervisorRpc,
    rpc_server_handle: Option<ServerHandle>,
    // TODO:: add other actors
}

impl SupervisorService {
    /// Creates a new Supervisor service instance.
    pub fn new(config: SupervisorServiceConfig) -> Result<Self> {
        // Initialize the core Supervisor logic
        // In the future, this might take configuration or client connections
        let supervisor_core = Arc::new(Supervisor::new());

        // Create the RPC implementation, sharing the core logic
        let rpc_impl = SupervisorRpc::new(supervisor_core.clone());

        Ok(Self {
            config,
            supervisor_core,
            rpc_impl,
            rpc_server_handle: None,
        })
    }

    /// Runs the Supervisor service.
    /// This function will typically run indefinitely until interrupted.
    pub async fn run(&mut self) -> Result<()> {
        info!(
            "Attempting to start Supervisor RPC server on {}",
            self.config.rpc_addr
        );

        let server = ServerBuilder::default()
            .build(self.config.rpc_addr)
            .await?;

        self.rpc_server_handle = Some(server.start(self.rpc_impl.clone().into_rpc()));

        info!(
            "Supervisor RPC server started successfully and listening on {}",
            self.config.rpc_addr
        );
        
        Ok(())
    }

    pub async fn shutdown(mut self) -> Result<()> {
        // TODO:: implement shutdown on each actors
        if let Some(handle) = self.rpc_server_handle.take() {
            info!("Sending stop signal to RPC server...");
            handle.stop()?; // Signal the server to stop accepting new connections
            info!("Waiting for RPC server to shut down completely...");
            handle.stopped().await; // Wait for the server to fully stop
            info!("Supervisor RPC server shut down gracefully.");
        } else {
            warn!("Shutdown called, but RPC server handle was not present. Was run() called?");
        }
        // TODO: Add shutdown logic for other components if any are added.
        Ok(())
    }
}