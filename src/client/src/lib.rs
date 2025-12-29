use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    ServerError(String),
    DeserializationError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ServerError(msg) => write!(f, "Server error: {}", msg),
            ApiError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
        }
    }
}

impl Error for ApiError {}

/// Node information object containing installation and network details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeObject {
    /// Client's IP Address
    pub client_ip: String,
    /// Installation path of the node
    pub install_path: String,
    /// Node's Local IP Address
    pub local_ip: String,
    /// WebSocket Address
    pub web_socket_addr: String,
}

impl NodeObject {
    /// Creates a new NodeObject instance
    pub fn new(
        client_ip: String,
        install_path: String,
        local_ip: String,
        web_socket_addr: String,
    ) -> Self {
        Self {
            client_ip,
            install_path,
            local_ip,
            web_socket_addr,
        }
    }
}

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    /// Creates a new API client with the base URL as current
    pub fn new() -> Self {
        Self {
            base_url: String::default(),
            client: reqwest::Client::new(),
        }
    }

    /// Creates a new API client with the base URL as current
    pub fn remote<S: Into<String>>(base_url: S) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Retrieves node information
    ///
    /// # Returns
    /// * `Ok(NodeObject)` - Node info retrieved successfully (200)
    /// * `Err(ApiError)` - Internal server error (500) or network error
    pub async fn node_info(&self) -> Result<NodeObject, ApiError> {
        let url = format!("{}/api/v0/node", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            200 => {
                let node = response
                    .json::<NodeObject>()
                    .await
                    .map_err(|e| ApiError::DeserializationError(e.to_string()))?;
                Ok(node)
            }
            500 => Err(ApiError::ServerError("Internal server error".to_string())),
            status => Err(ApiError::ServerError(format!(
                "Unexpected status code: {}",
                status
            ))),
        }
    }

    /// Registers a new user
    ///
    /// # Returns
    /// * `Ok(())` - User registered successfully (201)
    /// * `Err(ApiError)` - Internal server error (500) or network error
    pub async fn user_register(&self) -> Result<(), ApiError> {
        let url = format!("{}/api/v0/user", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            201 => Ok(()),
            500 => Err(ApiError::ServerError("Internal server error".to_string())),
            status => Err(ApiError::ServerError(format!(
                "Unexpected status code: {}",
                status
            ))),
        }
    }
}
