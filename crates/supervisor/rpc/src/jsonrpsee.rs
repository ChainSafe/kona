//! The Optimism Supervisor RPC API using `jsonrpsee`

use alloy_eips::eip1898::BlockNumHash; 
use kona_interop::{DerivedIdPair, SuperRootResponse};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use alloy_primitives::B256;
use kona_interop::{ExecutingDescriptor, SafetyLevel};

/// Optimism specified rpc interface.
///
/// TODO: add doc and interface reference
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "supervisor"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "supervisor"))]
pub trait SupervisorApi {
  /// Gets the derivedFrom BlockRef for a specific block
  #[method(name = "crossDerivedToSource")]
  async fn cross_derived_to_source(&self) -> RpcResult<()>;

  /// Gets the localUnsafe BlockId
  #[method(name= "localUnsafe")]
  async fn local_unsafe(&self) -> RpcResult<BlockNumHash>;

  /// Gets the crossSafe DerivedIdPair
  #[method(name= "crossSafe")]
  async fn cross_safe(&self) -> RpcResult<DerivedIdPair>;

  /// Gets the finalized BlockId
  #[method(name = "finalized")]
  async fn finalized(&self) -> RpcResult<BlockNumHash>;

  /// Gets the finalizedL1 BlockRef
  #[method(name = "finalizedL1")]
  async fn finalized_l1(&self) -> RpcResult<()>;

  /// Gets the super root state at a specified timestamp, which represents the global state across all monitored chains.
  #[method(name = "superRootAtTimestamp")]
  async fn super_root_at_timestamp(&self) -> RpcResult<SuperRootResponse>;

  /// Gets the supervisor status status
  #[method(name = "syncStatus")]
  async fn sync_status(&self) -> RpcResult<()>;

  /// Gets the last derived block for each chain, from the given L1 block
  #[method(name = "allSafeDerivedAt")]
  async fn all_safe_derived_at(&self) -> RpcResult<()>;

  /// Verifies if an access-list references only valid messages
  #[method(name = "checkAccessList")]
  async fn check_access_list(
    &self, 
    inbox_entries: Vec<B256>, 
    min_safety: SafetyLevel,
    executing_descriptor: ExecutingDescriptor,
  ) -> RpcResult<()>;
}
