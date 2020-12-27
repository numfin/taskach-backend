use crate::app_env::get_env;

use super::datastore_client::DatastoreClient;
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
    #[error("{0} not found")]
    NotFound(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    CreationError(String),
    #[error("{0}")]
    MutationError(String),
    #[error("{0}")]
    AuthError(String),
}
pub type Response<T> = Result<T, ResponseError>;

static SERVICE_URL: &str = "https://datastore.googleapis.com";

#[derive(Debug)]
pub enum ClientError {
    TokenError(gouth::Error),
    InvalidEmulatorHost(tonic::codegen::http::uri::InvalidUri),
    TlsConfigError(tonic::transport::Error),
    ConnectionError(tonic::transport::Error),
}

pub type Client = DatastoreClient<Channel>;

async fn get_channel() -> Result<tonic::transport::Endpoint, ClientError> {
    let channel: tonic::transport::Endpoint;

    let firebase_emulator_host = get_env::datastore_emulator_host();
    if let Some(host) = firebase_emulator_host {
        channel = Channel::from_shared(host).map_err(ClientError::InvalidEmulatorHost)?;
    } else {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name(&SERVICE_URL[8..]);
        channel = Channel::from_static(SERVICE_URL)
            .tls_config(tls_config)
            .map_err(ClientError::TlsConfigError)?
    }
    Ok(channel)
}

pub async fn create_service() -> Result<DatastoreClient<Channel>, ClientError> {
    let channel = get_channel()
        .await?
        .connect()
        .await
        .map_err(|err| ClientError::ConnectionError(err))?;

    let token = Token::new().map_err(|err| ClientError::TokenError(err))?;

    let client = DatastoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().expect("No token header");
        let meta =
            MetadataValue::from_str(token).expect("Cannot create request metadata from token");
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });
    Ok(client)
}
