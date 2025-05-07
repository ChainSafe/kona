//! This crate contains the core logic for the Optimism Supervisor component.

/// Contains the main Supervisor struct and its implementation.
mod supervisor;
pub use supervisor::Supervisor;

mod rpc;
pub use rpc::server::SupervisorRpc;