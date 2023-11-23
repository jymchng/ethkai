use anyhow::{anyhow, Ok, Result};
use ethers::providers::{Authorization, Http, Provider};
use lazy_static::lazy_static;
use secrecy::{ExposeSecret, SecretString};

use url::Url;

use ethers_rs_types::{init::types::{
    WhichAPIProvider, ALCHEMY_API_HTTP_URL, INFURA_API_HTTP_URL, MORALIS_API_HTTP_URL,
    QUICKNODE_API_HTTP_URL,}, error::{INITIALIZATION_ERROR_STRING,},
};

lazy_static! {
    pub static ref ALCHEMY_HTTP_PROVIDER: Provider<Http> =
        init_provider(&WhichAPIProvider::ALCHEMY).expect(&format!(
            "{}Cannot instantiate `AlchemyHttpProvider`",
            *INITIALIZATION_ERROR_STRING,
        ));
    pub static ref MORALIS_HTTP_PROVIDER: Provider<Http> =
        init_provider(&WhichAPIProvider::MORALIS).expect(&format!(
            "{}Cannot instantiate `MoralisHttpProvider`",
            *INITIALIZATION_ERROR_STRING,
        ));
    pub static ref INFURA_HTTP_PROVIDER: Provider<Http> = init_provider(&WhichAPIProvider::INFURA)
        .expect(&format!(
            "{}Cannot instantiate `InfuraHttpProvider`",
            *INITIALIZATION_ERROR_STRING,
        ));
    pub static ref QUICK_NODE_HTTP_PROVIDER: Provider<Http> =
        init_provider(&WhichAPIProvider::QUICKNODE).expect(&format!(
            "{}Cannot instantiate `QuicknodeHttpProvider`",
            *INITIALIZATION_ERROR_STRING,
        ));
}

pub fn init_provider(which_provider: &WhichAPIProvider) -> Result<Provider<Http>> {
    let api_secret = crate::io::utils::init_api_credentials(&which_provider)?;
    match which_provider {
        WhichAPIProvider::ALCHEMY => {
            init_provider_impl(ALCHEMY_API_HTTP_URL, api_secret, which_provider)
        }
        WhichAPIProvider::INFURA => {
            init_provider_impl(INFURA_API_HTTP_URL, api_secret, which_provider)
        }
        WhichAPIProvider::QUICKNODE => {
            init_provider_impl(QUICKNODE_API_HTTP_URL, api_secret, which_provider)
        }
        WhichAPIProvider::MORALIS => init_moralis_http_provider(MORALIS_API_HTTP_URL, api_secret),
        _ => unreachable!(),
    }
}

fn init_provider_impl(
    url_: &str,
    api_secret: SecretString,
    which_provider: &WhichAPIProvider,
) -> Result<Provider<Http>> {
    let provider_url = SecretString::new(format!("{}{}", url_, api_secret.expose_secret()));
    Provider::try_from(provider_url.expose_secret()).map_err(|err| {
        anyhow!(
            "{}Unable to get a `Provider` from {}, {err}",
            *INITIALIZATION_ERROR_STRING,
            &which_provider
        )
    })
}

// Special because it needs authorization.
fn init_moralis_http_provider(url_: &str, api_secret: SecretString) -> Result<Provider<Http>> {
    let authorization = Authorization::Bearer(api_secret.expose_secret().to_string());
    let url = Url::parse(url_)?;
    let http = Http::new_with_auth(url, authorization)?;
    Ok(Provider::new(http))
}
