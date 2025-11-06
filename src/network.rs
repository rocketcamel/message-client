use std::error::Error;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

use crate::components::Message;

pub enum NetworkRequest {
    Authenticate(AuthRequest),
    SendMessage { content: String },
    FetchMessages,
    RefreshToken,
}

pub enum NetworkResponse {
    AuthSuccess { token: String },
    AuthError { error: String },
    MessageSent,
    MessagesReceived { messages: Vec<Message> },
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("error creating auth request")]
    Creation,
    #[error("error sending request")]
    Request(String),
    #[error("error deserializing response")]
    Deserialize(String),
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
    user_id: String,
    expiry: u32,
}

pub struct NetworkTask {
    client: reqwest::Client,
    base_url: String,
}

impl NetworkTask {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: std::env::var("BASE_URL").unwrap_or(
                "http://ec2-44-250-68-143.us-west-2.compute.amazonaws.com:8000".to_string(),
            ),
        }
    }

    pub async fn run(
        &self,
        mut req_rx: mpsc::UnboundedReceiver<NetworkRequest>,
        resp_tx: mpsc::UnboundedSender<NetworkResponse>,
    ) {
        while let Some(req) = req_rx.recv().await {
            match req {
                NetworkRequest::Authenticate(auth_req) => match self.auth(&auth_req).await {
                    Ok(token) => {
                        resp_tx.send(NetworkResponse::AuthSuccess { token }).ok();
                    }
                    Err(e) => {
                        resp_tx
                            .send(NetworkResponse::AuthError {
                                error: e.to_string(),
                            })
                            .ok();
                    }
                },
                NetworkRequest::SendMessage { content: _ } => todo!(),
                NetworkRequest::FetchMessages => todo!(),
                NetworkRequest::RefreshToken => todo!(),
            }
        }
    }

    async fn auth(&self, auth_req: &AuthRequest) -> Result<String, AuthError> {
        let body = serde_json::to_string(auth_req).map_err(|_| AuthError::Creation)?;
        let response = self
            .client
            .post(format!("{}/auth/login", self.base_url))
            .body(body)
            .send()
            .await
            .map_err(|e| AuthError::Request(e.to_string()))?
            .json::<AuthResponse>()
            .await
            .map_err(|e| AuthError::Deserialize(e.to_string()))?;
        Ok(response.token)
    }
}
