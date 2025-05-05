//! Server-side implementation of the Supervisor RPC API.

use crate::SupervisorApiServer;
use alloy_eips::eip1898::BlockNumHash;
use async_trait::async_trait;
use jsonrpsee::{
    core::{Error as RpcError, RpcResult},
    types::{error::ErrorCode, ErrorObject},
};
use kona_interop::{DerivedIdPair, SuperRootResponse};
use kona_supervisor_core::supervisor::Supervisor;
use std::sync::Arc;

/// A helper function to map core errors to RPC errors.
/// TODO: Replace `std::fmt::Error` with a proper custom error type in core
/// and provide more specific error mapping here.
fn map_err(err: std::fmt::Error) -> RpcError {
    // Using `std::fmt::Error` doesn't give us much info, so map to InternalError for now.
    // In a real implementation, match on specific error variants from the core crate.
    let error_object = ErrorObject::owned(
        ErrorCode::InternalError.code(),
        format!("Supervisor core error: {}", err), // Use format! as Error doesn't impl Display directly sometimes
        None::<()>,
    );
    RpcError::Call(error_object)
}

/// The server-side implementation struct for the `SupervisorApi`.
/// It holds a reference to the core Supervisor logic.
#[derive(Debug, Clone)]
pub struct SupervisorRpc {
    /// Reference to the core Supervisor logic.
    /// Using Arc allows sharing the Supervisor instance if needed,
    /// similar to how actors might share state or communication channels.
    supervisor: Arc<Supervisor>,
}

impl SupervisorRpc {
    /// Creates a new [`SupervisorRpc`] instance.
    /// This is where you would inject the actual Supervisor instance.
    pub fn new(supervisor: Arc<Supervisor>) -> Self {
        Self { supervisor }
    }
}

#[async_trait]
impl SupervisorApiServer for SupervisorRpc {
    async fn cross_derived_to_source(&self) -> RpcResult<()> {
        // Note: The core Supervisor::cross_derived_to_source likely needs arguments.
        // This needs alignment between the RPC API definition and the core logic.
        // For now, calling the placeholder static method. In reality, you'd call:
        // self.supervisor.cross_derived_to_source(args...).await.map_err(map_err)
        Supervisor::cross_derived_to_source().await.map_err(map_err)
    }

    async fn local_unsafe(&self) -> RpcResult<BlockNumHash> {
        Supervisor::local_unsafe().await.map_err(map_err)
    }

    async fn cross_safe(&self) -> RpcResult<DerivedIdPair> {
        Supervisor::cross_safe().await.map_err(map_err)
    }

    async fn finalized(&self) -> RpcResult<BlockNumHash> {
        Supervisor::finalized().await.map_err(map_err)
    }

    async fn finalized_l1(&self) -> RpcResult<()> {
        Supervisor::finalized_l1().await.map_err(map_err)
    }

    async fn super_root_at_timestamp(&self) -> RpcResult<SuperRootResponse> {
        Supervisor::super_root_at_timestamp().await.map_err(map_err)
    }

    async fn sync_status(&self) -> RpcResult<()> {
        Supervisor::sync_status().await.map_err(map_err)
    }

    async fn all_safe_derived_at(&self) -> RpcResult<()> {
        Supervisor::all_safe_derived_at().await.map_err(map_err)
    }

    async fn check_access_list(&self) -> RpcResult<()> {
        Supervisor::check_access_list().await.map_err(map_err)
    }
}