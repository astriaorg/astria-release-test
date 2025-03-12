mod frost;
mod sequencer_key;

use astria_core::generated::astria::signer::v1::frost_participant_service_client::FrostParticipantServiceClient;
use astria_eyre::{
    eyre,
    eyre::WrapErr as _,
};
use frost_ed25519::keys::PublicKeyPackage;
use http::Uri;

pub(crate) enum Signer {
    Single(Box<sequencer_key::SequencerKey>),
    Threshold(frost::FrostSigner),
}

impl Signer {
    pub(crate) fn address(&self) -> &astria_core::primitive::v1::Address {
        match self {
            Self::Single(signer) => signer.address(),
            Self::Threshold(signer) => signer.address(),
        }
    }

    pub(crate) async fn sign(
        &self,
        tx: astria_core::protocol::transaction::v1::TransactionBody,
    ) -> eyre::Result<astria_core::protocol::transaction::v1::Transaction> {
        match self {
            Self::Single(signer) => Ok(signer.sign(tx)),
            Self::Threshold(signer) => signer.sign(tx).await,
        }
    }
}

pub(crate) fn make_signer(
    no_frost_threshold_signing: bool,
    frost_min_signers: usize,
    frost_public_key_package_path: String,
    frost_participant_endpoints: String,
    sequencer_key_path: String,
    sequencer_address_prefix: String,
) -> eyre::Result<Signer> {
    let signer = if no_frost_threshold_signing {
        Signer::Single(Box::new(
            sequencer_key::SequencerKey::builder()
                .path(sequencer_key_path)
                .prefix(sequencer_address_prefix)
                .try_build()
                .wrap_err("failed to load sequencer private key")?,
        ))
    } else {
        let public_key_package = read_frost_key(frost_public_key_package_path)?;
        let frost_participant_endpoints: Vec<Uri> = frost_participant_endpoints
            .split(',')
            .map(str::to_string)
            .map(|s| s.parse().wrap_err("failed to parse participant endpoint"))
            .collect::<eyre::Result<Vec<Uri>>>()?;
        let participant_clients = frost_participant_endpoints
            .into_iter()
            .map(|endpoint| {
                FrostParticipantServiceClient::new(
                    tonic::transport::Endpoint::from(endpoint).connect_lazy(),
                )
            })
            .collect();
        Signer::Threshold(
            frost::FrostSignerBuilder::new()
                .min_signers(frost_min_signers)
                .public_key_package(public_key_package)
                .participant_clients(participant_clients)
                .address_prefix(sequencer_address_prefix)
                .try_build()
                .wrap_err("failed to initialize frost signer")?,
        )
    };
    Ok(signer)
}

fn read_frost_key<P: AsRef<std::path::Path>>(
    path: P,
) -> astria_eyre::eyre::Result<PublicKeyPackage> {
    let key_str =
        std::fs::read_to_string(path).wrap_err("failed to read frost public key package")?;
    serde_json::from_str::<PublicKeyPackage>(&key_str)
        .wrap_err("failed to deserialize public key package")
}
