use clap::{ Args};

/// Global configuration arguments.
#[derive(Args, Debug)]
pub struct SupervisorArgs {
    /// L1 RPC source.
    #[arg(
        long, 
        env = "L1_RPC", 
        help = "L1 RPC source",
    )]
    pub l1_rpc: String,

    /// L2 consensus rollup node RPC addresses.
    #[arg(
        long = "l2-consensus.nodes", 
        env = "L2_CONSENSUS_NODES", 
        help = "L2 consensus node RPCs", 
        value_delimiter = ',',
    )]
    pub l2_consensus_nodes: Vec<String>,

     /// JWT secrets for L2 consensus nodes.
    #[arg(
        long = "l2-consensus.jwt-secret", 
        env = "L2_CONSENSUS_JWT_SECRET", 
        help = "JWT secrets for L2 consensus nodes", 
        value_delimiter = ',',
    )]
    pub l2_consensus_jwt_secret: Vec<String>,

    /// Directory to store supervisor data.
    #[arg(
        long, 
        env = "DATADIR", 
        help = "Directory to store data",
    )]
    pub datadir: String,

    /// Optional endpoint to sync data from another supervisor.
    #[arg(
        long = "datadir.sync-endpoint", 
        env = "DATADIR_SYNC_ENDPOINT", 
        help = "Sync endpoint for supervisor",
    )]
    pub datadir_sync_endpoint: Option<String>,

    /// Path to the dependency-set JSON config file.
    #[arg(
        long = "dependency-set", 
        env = "DEPENDENCY_SET", 
        help = "Path to dependency-set config",
    )]
    pub dependency_set: Option<String>,
}