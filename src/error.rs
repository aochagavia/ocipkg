use crate::Digest;
use oci_spec::OciSpecError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    //
    // Invalid user input
    //
    #[error("Invalid digest: {0}")]
    InvalidDigest(String),
    #[error("Invalid name for repository: {0}")]
    InvalidName(String),
    #[error(transparent)]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Invalid reference to image: {0}")]
    InvalidReference(String),
    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),

    //
    // Invalid container image
    //
    #[error("Unknown digest in oci-archive: {0}")]
    UnknownDigest(Digest),
    #[error("No index.json is included in oci-archive")]
    MissingIndex,
    #[error("index.json does not have image name in manifest annotation")]
    MissingManifestName,
    #[error(transparent)]
    InvalidJson(#[from] serde_json::error::Error),

    //
    // System error
    //
    #[error("No valid home directory path could be retrieved from the operating system.")]
    NoValidHomeDirecotry,
    #[error(transparent)]
    UnknownIo(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<OciSpecError> for Error {
    fn from(e: OciSpecError) -> Self {
        match e {
            OciSpecError::SerDe(e) => Error::InvalidJson(e),
            OciSpecError::Io(e) => Error::UnknownIo(e),
            OciSpecError::Builder(_) => unreachable!(),
            OciSpecError::Other(e) => panic!("Unknown error within oci_spec: {}", e),
        }
    }
}
