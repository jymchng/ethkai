use ethers_rs_types::{init::types::{WhichAPIProvider, ALCHEMY_API_WS_URL, INFURA_API_WS_URL}, error::INITIALIZATION_ERROR_STRING};
use anyhow::{anyhow, Result};
use ethers::providers::{Provider, Ws};
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;

pub async fn init_provider(which_provider: &WhichAPIProvider) -> Result<Provider<Ws>> {
    let api_secret = crate::io::utils::init_api_credentials(&which_provider)?;
    match which_provider {
        WhichAPIProvider::INFURA => {
            init_provider_impl(INFURA_API_WS_URL, api_secret, which_provider).await
        }
        WhichAPIProvider::ALCHEMY => {
            init_provider_impl(ALCHEMY_API_WS_URL, api_secret, which_provider).await
        }
        _ => unreachable!(),
    }
}

async fn init_provider_impl(
    url_: &str,
    api_secret: SecretString,
    which_provider: &WhichAPIProvider,
) -> Result<Provider<Ws>> {
    let provider_url = SecretString::new(format!("{}{}", url_, api_secret.expose_secret()));
    let ws = Provider::<Ws>::connect(provider_url.expose_secret())
        .await
        .map_err(|err| {
            anyhow!(
                "{}Unable to get a `Provider` from {}, {err}",
                *INITIALIZATION_ERROR_STRING,
                &which_provider
            )
        });
    ws
}
