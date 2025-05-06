use clap::{ ArgAction, Args};
use kona_cli::init_tracing_subscriber;
use tracing_subscriber::EnvFilter;

/// Global configuration arguments.
#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Verbosity level (0-5).
    /// If set to 0, no logs are printed.
    /// By default, the verbosity level is set to 3 (info level).
    #[arg(long, short, global = true, default_value = "3", action = ArgAction::Count)]
    pub v: u8,
}

impl GlobalArgs {
    /// Initializes the telemetry stack and Prometheus metrics recorder.
    pub fn init_tracing(&self, filter: Option<EnvFilter>) -> anyhow::Result<()> {
        Ok(init_tracing_subscriber(self.v, filter)?)
    }
}
