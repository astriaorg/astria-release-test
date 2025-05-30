use color_eyre::eyre;

/// Main entry point for the Astria CLI application.
/// Initializes logging and runs the CLI interface.
#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(std::io::stderr)
        .init();

    astria_cli::Cli::run().await
}
