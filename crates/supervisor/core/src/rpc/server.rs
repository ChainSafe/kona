//! Server-side implementation of the Supervisor RPC API.

use alloy_eips::eip1898::BlockNumHash;
use async_trait::async_trait;
use jsonrpsee::{
    core::RpcResult, // Import RpcError alias
    types::{error::ErrorCode, ErrorObject},
};
use kona_interop::{DerivedIdPair, SuperRootResponse};
use crate::supervisor::Supervisor;
use kona_supervisor_rpc::SupervisorApiServer;
use std::sync::Arc;

/// The server-side implementation struct for the `SupervisorApi`.
/// It holds a reference to the core Supervisor logic.
#[derive(Debug)]
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
        // This needs alignment between the RPC API definition and the core logic (add args here).
        // self.supervisor.cross_derived_to_source()
        //   .await
        //   .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn local_unsafe(&self) -> RpcResult<BlockNumHash> {
        // self.supervisor.local_unsafe()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn cross_safe(&self) -> RpcResult<DerivedIdPair> {
        // self.supervisor.cross_safe()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn finalized(&self) -> RpcResult<BlockNumHash> {
        // self.supervisor.finalized()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn finalized_l1(&self) -> RpcResult<()> {
        // self.supervisor.finalized_l1()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn super_root_at_timestamp(&self) -> RpcResult<SuperRootResponse> {
        // Note: Needs arguments aligned with core logic.
        // self.supervisor.super_root_at_timestamp()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn sync_status(&self) -> RpcResult<()> {
        // self.supervisor.sync_status()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn all_safe_derived_at(&self) -> RpcResult<()> {
        // Note: Needs arguments aligned with core logic.
        // self.supervisor.all_safe_derived_at()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }

    async fn check_access_list(&self) -> RpcResult<()> {
        // Note: Needs arguments aligned with core logic.
        // self.supervisor.check_access_list()
        // .await
        // .map_err(|_| ErrorObject::from(ErrorCode::InternalError))?;
        Err(ErrorObject::from(ErrorCode::MethodNotFound))
    }
}