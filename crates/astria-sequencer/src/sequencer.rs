use astria_eyre::{
    anyhow_to_eyre,
    eyre::{
        eyre,
        OptionExt as _,
        Result,
        WrapErr as _,
    },
};
use penumbra_tower_trace::{
    trace::request_span,
    v038::RequestExt as _,
};
use telemetry::metrics::register_histogram_global;
use tendermint::v0_38::abci::ConsensusRequest;
use tokio::{
    select,
    signal::unix::{
        signal,
        SignalKind,
    },
    sync::watch,
};
use tower_abci::v038::Server;
use tracing::{
    error,
    error_span,
    info,
    info_span,
};

use crate::{
    app::App,
    config::Config,
    mempool::Mempool,
    metrics::Metrics,
    service,
};

pub struct Sequencer;

impl Sequencer {
    #[expect(clippy::missing_errors_doc, reason = "not a public function")]
    pub async fn run_until_stopped(config: Config, metrics: &'static Metrics) -> Result<()> {
        cnidarium::register_metrics();
        register_histogram_global("cnidarium_get_raw_duration_seconds");
        register_histogram_global("cnidarium_nonverifiable_get_raw_duration_seconds");
        let span = info_span!("Sequencer::run_until_stopped");

        if config
            .db_filepath
            .try_exists()
            .context("failed checking for existence of db storage file")?
        {
            span.in_scope(|| {
                info!(
                    path = %config.db_filepath.display(),
                    "opening storage db"
                );
            });
        } else {
            span.in_scope(|| {
                info!(
                    path = %config.db_filepath.display(),
                    "creating storage db"
                );
            });
        }

        let mut signals = spawn_signal_handler();

        let substore_prefixes = vec![penumbra_ibc::IBC_SUBSTORE_PREFIX];

        let storage = cnidarium::Storage::load(
            config.db_filepath.clone(),
            substore_prefixes
                .into_iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
        .await
        .map_err(anyhow_to_eyre)
        .wrap_err("failed to load storage backing chain state")?;
        let snapshot = storage.latest_snapshot();

        let mempool = Mempool::new(metrics, config.mempool_parked_max_tx_count);

        let app = App::new(snapshot, mempool.clone(), metrics)
            .await
            .wrap_err("failed to initialize app")?;

        let event_bus_subscription = app.subscribe_to_events();

        let consensus_service = tower::ServiceBuilder::new()
            .layer(request_span::layer(|req: &ConsensusRequest| {
                req.create_span()
            }))
            .service(tower_actor::Actor::new(10, |queue: _| {
                let storage = storage.clone();
                async move { service::Consensus::new(storage, app, queue).run().await }
            }));
        let mempool_service = service::Mempool::new(storage.clone(), mempool.clone(), metrics);
        let info_service =
            service::Info::new(storage.clone()).wrap_err("failed initializing info service")?;
        let snapshot_service = service::Snapshot;

        let server = Server::builder()
            .consensus(consensus_service)
            .info(info_service)
            .mempool(mempool_service)
            .snapshot(snapshot_service)
            .finish()
            .ok_or_eyre("server builder didn't return server; are all fields set?")?;

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        let (server_exit_tx, server_exit_rx) = tokio::sync::oneshot::channel();

        let grpc_addr = config
            .grpc_addr
            .parse()
            .wrap_err("failed to parse grpc_addr address")?;
        let grpc_server_handle = crate::grpc::start_server(
            &storage,
            mempool,
            grpc_addr,
            config.no_optimistic_blocks,
            event_bus_subscription,
            shutdown_rx,
        );

        span.in_scope(|| info!(config.listen_addr, "starting sequencer"));
        let server_handle = tokio::spawn(async move {
            match server.listen_tcp(&config.listen_addr).await {
                Ok(()) => {
                    // this shouldn't happen, as there isn't a way for the ABCI server to exit
                    info_span!("abci_server").in_scope(|| info!("ABCI server exited successfully"));
                }
                Err(e) => {
                    error_span!("abci_server")
                        .in_scope(|| error!(err = e.as_ref(), "ABCI server exited with error"));
                }
            }
            let _ = server_exit_tx.send(());
        });

        select! {
            _ = signals.stop_rx.changed() => {
                span.in_scope(|| info!("shutting down sequencer"));
            }

            _ = server_exit_rx => {
                span.in_scope(|| error!("ABCI server task exited, this shouldn't happen"));
            }
        }

        shutdown_tx
            .send(())
            .map_err(|()| eyre!("failed to send shutdown signal to grpc server"))?;
        grpc_server_handle
            .await
            .wrap_err("grpc server task failed")?
            .wrap_err("grpc server failed")?;
        server_handle.abort();
        Ok(())
    }
}

struct SignalReceiver {
    stop_rx: watch::Receiver<()>,
}

fn spawn_signal_handler() -> SignalReceiver {
    let (stop_tx, stop_rx) = watch::channel(());
    tokio::spawn(async move {
        let mut sigint = signal(SignalKind::interrupt()).expect(
            "setting a SIGINT listener should always work on unix; is this running on unix?",
        );
        let mut sigterm = signal(SignalKind::terminate()).expect(
            "setting a SIGTERM listener should always work on unix; is this running on unix?",
        );
        loop {
            select! {
                _ = sigint.recv() => {
                    info!("received SIGINT");
                    let _ = stop_tx.send(());
                }
                _ = sigterm.recv() => {
                    info!("received SIGTERM");
                    let _ = stop_tx.send(());
                }
            }
        }
    });

    SignalReceiver {
        stop_rx,
    }
}
