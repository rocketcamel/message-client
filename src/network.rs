use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

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

#[derive(Debug, Deserialize)]
pub struct Token {
    pub token: String,
    pub user_id: u32,
    pub expiry: u64,
}

impl Token {
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        return now >= self.expiry;
    }
}

pub enum NetworkResponse {
    AuthSuccess(Token),
    AuthError { error: String },
    MessageSent,
    MessagesReceived { messages: Vec<Message> },
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("error with request")]
    Request(#[from] reqwest::Error),
    #[error("error deserializing response")]
    Deserialize(#[from] serde_json::Error),
    #[error("error with response status: {status}, body: {body}")]
    Status { status: String, body: String },
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
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
                        resp_tx.send(NetworkResponse::AuthSuccess(token)).ok();
                    }
                    Err(e) => {
                        resp_tx
                            .send(NetworkResponse::AuthError {
                                error: format!("{e:?}"),
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

    async fn auth(&self, auth_req: &AuthRequest) -> Result<Token, AuthError> {
        let response = self
            .client
            .post(format!("{}/auth/login", self.base_url))
            .json(auth_req)
            .send()
            .await?;

        if let Err(e) = response.error_for_status_ref() {
            let response_text = response.text().await?;

            return Err(AuthError::Status {
                status: format!("{e:?}"),
                body: response_text,
            });
        }

        let des_response = response.json::<Token>().await?;
        Ok(des_response)
    }
}
