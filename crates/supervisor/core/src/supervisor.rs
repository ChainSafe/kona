use alloy_eips::eip1898::BlockNumHash; 
use alloy_primitives::B256;
use kona_interop::{ExecutingDescriptor, SafetyLevel, DerivedIdPair, SuperRootResponse};
use thiserror::Error;
use core::error;

/// Custom error type for the Supervisor core logic.
#[derive(Debug, Error)]
pub enum SupervisorError {
    /// Indicates that a feature or method is not yet implemented.
    #[error("functionality not implemented")]
    Unimplemented,
}

/// The core Supervisor component responsible for monitoring and coordinating chain states.
#[derive(Debug)]
pub struct Supervisor {
}

impl Supervisor {
  /// Creates a new [`Supervisor`] instance.
  pub fn new() -> Self {
    Self {}
  }

  /// Gets the derivedFrom BlockRef for a specific block
  pub async fn cross_derived_to_source(&self) -> Result<(), SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the localUnsafe BlockId
  pub async fn local_unsafe(&self) -> Result<BlockNumHash, SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the crossSafe DerivedIdPair
  pub async fn cross_safe(&self) -> Result<DerivedIdPair, SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the finalized BlockId
  pub async fn finalized(&self) -> Result<BlockNumHash, SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the finalizedL1 BlockRef
  pub async fn finalized_l1(&self) -> Result<(), SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the super root state at a specified timestamp, which represents the global state across all monitored chains.
  pub async fn super_root_at_timestamp(&self) -> Result<SuperRootResponse, SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the supervisor status status
  pub async fn sync_status(&self) -> Result<(), SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Gets the last derived block for each chain, from the given L1 block
  pub async fn all_safe_derived_at(&self) -> Result<(), SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }

  /// Verifies if an access-list references only valid messages
  pub async fn check_access_list(
    &self, 
    _inbox_entries: Vec<B256>,
    _min_safety: SafetyLevel,
    _executing_descriptor: ExecutingDescriptor,
  ) -> Result<(), SupervisorError> {
    Err(SupervisorError::Unimplemented)
  }
}