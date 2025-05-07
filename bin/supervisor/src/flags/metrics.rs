use std::net::IpAddr;
use clap::{ Args};
use kona_cli::init_prometheus_server;

/// Configuration for Prometheus metrics.
#[derive(Args, Debug)]
pub struct MetricsArgs {
    /// Controls whether prometheus metrics are enabled.
    /// Disabled by default.
    #[arg(
        long = "metrics.enabled",
        global = true,
        default_value_t = false,
        env = "METRICS_ENABLED"
    )]
    pub enabled: bool,
    /// The port to serve prometheus metrics on
    #[arg(
        long = "metrics.port",
        global = true,
        default_value = "9090",
        env = "METRICS_PORT"
    )]
    pub port: u16,
    /// The ip address to use to emit prometheus metrics.
    #[arg(
        long = "metrics.addr",
        global = true,
        default_value = "0.0.0.0",
        env = "METRICS_ADDR"
    )]
    pub addr: IpAddr,
}

impl MetricsArgs {
    /// Initialize the tracing stack and Prometheus metrics recorder.
    ///
    /// This function should be called at the beginning of the program.
    pub fn init_metrics(&self) -> anyhow::Result<()> {
        if self.enabled {
            init_prometheus_server(self.addr, self.port)?;
            // TODO:: initialize metrics
        }

        Ok(())
    }
}