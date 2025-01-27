use astria_core::primitive::v1::asset;
use serde::{
    Deserialize,
    Serialize,
};

// Allowed `struct_excessive_bools` because this is used as a container
// for deserialization. Making this a builder-pattern is not actionable.
#[expect(
    clippy::struct_excessive_bools,
    reason = "represents a config with flags"
)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// The single config for creating an astria-auctioneer service.
pub struct Config {
    /// The endpoint for the sequencer gRPC service used for the optimistic block stream
    pub sequencer_grpc_endpoint: String,
    /// The endpoint for the rollup gRPC service used for the optimistic execution and bundle
    /// streams
    pub rollup_grpc_endpoint: String,
    /// The rollup ID used to filter the optimistic blocks stream
    pub rollup_id: String,
    /// Log level for the service.
    pub log: String,
    /// Forces writing trace data to stdout no matter if connected to a tty or not.
    pub force_stdout: bool,
    /// Disables writing trace data to an opentelemetry endpoint.
    pub no_otel: bool,
    /// Set to true to disable the metrics server
    pub no_metrics: bool,
    /// The endpoint which will be listened on for serving prometheus metrics
    pub metrics_http_listener_addr: String,
    /// Writes a human readable format to stdout instead of JSON formatted OTEL trace data.
    pub pretty_print: bool,
}

impl config::Config for Config {
    const PREFIX: &'static str = "ASTRIA_AUCTIONEER_";
}

#[cfg(test)]
mod tests {
    use super::Config;

    const EXAMPLE_ENV: &str = include_str!("../local.env.example");

    #[test]
    fn example_env_config_is_up_to_date() {
        config::tests::example_env_config_is_up_to_date::<Config>(EXAMPLE_ENV);
    }
}
