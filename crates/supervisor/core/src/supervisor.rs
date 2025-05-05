use std::fmt::Error;

use alloy_eips::eip1898::BlockNumHash; 
use kona_interop::{DerivedIdPair, SuperRootResponse};

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
  pub async fn cross_derived_to_source(&self) -> Result<(), Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the localUnsafe BlockId
  pub async fn local_unsafe(&self) -> Result<BlockNumHash, Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the crossSafe DerivedIdPair
  pub async fn cross_safe(&self) -> Result<DerivedIdPair, Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the finalized BlockId
  pub async fn finalized(&self) -> Result<BlockNumHash, Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the finalizedL1 BlockRef
  pub async fn finalized_l1(&self) -> Result<(), Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the super root state at a specified timestamp, which represents the global state across all monitored chains.
  pub async fn super_root_at_timestamp(&self) -> Result<SuperRootResponse, Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the supervisor status status
  pub async fn sync_status(&self) -> Result<(), Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Gets the last derived block for each chain, from the given L1 block
  pub async fn all_safe_derived_at(&self) -> Result<(), Error> {
    unimplemented!("This method is not implemented yet")
  }

  /// Verifies if an access-list references only valid messages
  pub async fn check_access_list(&self) -> Result<(), Error> {
    unimplemented!("This method is not implemented yet")
  }
}