use crate::app_env::get_env;

use super::datastore_client::DatastoreClient;
use googapis::CERTIFICATES;
use gouth::{self, Builder, Token};
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
    Placeholder,
}

pub type Client = DatastoreClient<Channel>;

async fn get_channel(datastore_host: &Option<String>) -> Result<Channel, ClientError> {
    let mut client: Result<Channel, ClientError> = Err(ClientError::Placeholder);

    if let Some(host) = datastore_host.clone() {
        client = Channel::from_shared(host)
            .map_err(ClientError::InvalidEmulatorHost)?
            .connect()
            .await
            .map_err(ClientError::ConnectionError);
    }
    if client.is_err() {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name(&SERVICE_URL[8..]);
        client = Channel::from_static(SERVICE_URL)
            .tls_config(tls_config)
            .map_err(ClientError::TlsConfigError)?
            .connect()
            .await
            .map_err(ClientError::ConnectionError);
    }

    Ok(client?)
}

fn get_token(use_env: bool) -> gouth::Result<Token> {
    if !use_env {
        if let Ok(token) = Builder::new()
            .file("~/.config/gcloud/application_default_credentials.json")
            .build()
        {
            return Ok(token);
        }
    }
    Token::new()
}

pub async fn create_service() -> Result<DatastoreClient<Channel>, ClientError> {
    let datastore_host = &get_env::datastore_emulator_host();
    let channel = get_channel(datastore_host).await?;

    let token = get_token(datastore_host.is_some()).map_err(ClientError::TokenError)?;

    let client = DatastoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().expect("No token header");
        let meta =
            MetadataValue::from_str(token).expect("Cannot create request metadata from token");
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });
    Ok(client)
}
