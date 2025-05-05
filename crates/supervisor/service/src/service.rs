//! Contains the main Supervisor service runner.

use anyhow::Result;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use kona_supervisor_core::{
  Supervisor,
  SupervisorRpc,
};
use kona_supervisor_rpc::SupervisorApiServer;
use std::{net::SocketAddr, sync::Arc};

/// Configuration for the Supervisor service.
#[derive(Debug, Clone)]
pub struct SupervisorServiceConfig {
    /// The socket address for the RPC server to listen on.
    pub rpc_addr: SocketAddr,
    // Add other configuration fields as needed (e.g., connection details for L1/L2 nodes)
}

/// The main Supervisor service structure.
#[derive(Debug)]
pub struct SupervisorService {
    config: SupervisorServiceConfig,
    supervisor_core: Arc<Supervisor>,
    rpc_impl: SupervisorRpc,
    // Optional: Add handles or channels for managing background tasks if needed
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
        })
    }

    /// Runs the Supervisor service.
    /// This function will typically run indefinitely until interrupted.
    pub async fn run(self) -> Result<ServerHandle> {
        println!("Starting Supervisor RPC server on {}", self.config.rpc_addr);

        let server = ServerBuilder::default()
            .build(self.config.rpc_addr)
            .await?;

        let handle = server.start(self.rpc_impl.into_rpc());

        println!("Supervisor RPC server started successfully.");
        // In a real service, you might spawn other background tasks here
        // and wait for them or the server handle.
        Ok(handle)
    }
}