use super::firestore_client::FirestoreClient;
use googapis::CERTIFICATES;
use gouth::Token;
use thiserror::Error;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[derive(Error, Debug, Clone)]
pub enum ResponseError {
    #[error("Error happened while {0}")]
    UnexpectedError(String),
    #[error("Document {0} not found")]
    NotFound(String),
    #[error("Document {0} {1} already exists")]
    AlreadyExists(String, String),
    #[error("{0}")]
    CreationError(String),
    #[error("{0}")]
    AuthError(String),
}
pub type Response<T> = Result<T, ResponseError>;

pub static PARENT: &str = "projects/taskach-2/databases/(default)/documents";
static SERVICE_URL: &str = "https://firestore.googleapis.com";

#[derive(Debug)]
pub enum ClientError {
    TokenError(gouth::Error),
    InvalidEmulatorHost(tonic::codegen::http::uri::InvalidUri),
    TlsConfigError(tonic::transport::Error),
    ConnectionError(tonic::transport::Error),
}

pub type Client = FirestoreClient<Channel>;

pub async fn create_service() -> Result<FirestoreClient<Channel>, ClientError> {
    let token = Token::new().map_err(|err| ClientError::TokenError(err))?;
    let channel: tonic::transport::Endpoint;

    if let Ok(host) = std::env::var("FIRESTORE_EMULATOR_HOST") {
        channel =
            Channel::from_shared(host).map_err(|err| ClientError::InvalidEmulatorHost(err))?;
    } else {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name(&SERVICE_URL[8..]);
        channel = Channel::from_static(SERVICE_URL)
            .tls_config(tls_config)
            .map_err(|err| ClientError::TlsConfigError(err))?
    }

    let channel = channel
        .connect()
        .await
        .map_err(|err| ClientError::ConnectionError(err))?;

    let client = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().expect("No token header");
        let meta =
            MetadataValue::from_str(token).expect("Cannot create request metadata from token");
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });
    Ok(client)
}
