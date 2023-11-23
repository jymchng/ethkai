use colored::Colorize;
use lazy_static::lazy_static;
use text_colorizer::ColoredString;
use thiserror::Error;

lazy_static! {
    pub static ref INITIALIZATION_ERROR_STRING: ColoredString =
        "INITIALIZATION ERROR: ".to_string().red().bold();
    pub static ref RED_ERROR_STRING: ColoredString = "ERROR: ".to_string().red().bold();
}

#[derive(Error, Debug)]
pub enum EthersRSUtilsError {
    #[error("{}{provider}-provider has an initialization error", *INITIALIZATION_ERROR_STRING, provider = if *http {"http"} else {"ws"})]
    InitializationError { http: bool, ws: bool },
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}
