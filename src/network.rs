use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

use crate::components::{Message, MessageSender};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ServerMessage {
    pub id: u32,
    pub body: String,
    pub user_id: u32,
    pub in_reply_to: Option<u32>,
    pub channel: String,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
pub enum NetworkRequest {
    Authenticate(AuthRequest),
    SendMessage { content: String },
    FetchMessages,
    RefreshToken,
}

#[allow(dead_code)]
pub enum NetworkResponse {
    Auth(Token),
    MessageSent,
    MessagesReceived(Vec<Message>),
    Error(NetworkError),
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("error occured with request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("error deserializing: {0}")]
    Deserialize(#[source] reqwest::Error),
    #[error("error authenticating: {0}")]
    Auth(#[from] AuthError),
}

#[derive(Debug, Error)]
pub(crate) enum AuthError {
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
                        resp_tx.send(NetworkResponse::Auth(token)).ok();
                    }
                    Err(e) => {
                        resp_tx
                            .send(NetworkResponse::Error(NetworkError::Auth(e)))
                            .ok();
                    }
                },
                NetworkRequest::SendMessage { content: _ } => todo!(),
                NetworkRequest::FetchMessages => match self.fetch_messages().await {
                    Ok(messages) => {
                        resp_tx
                            .send(NetworkResponse::MessagesReceived(
                                messages
                                    .iter()
                                    .map(|m| Message {
                                        timestamp: m.created_at,
                                        sender: MessageSender::User(m.user_id),
                                        content: m.body.clone(),
                                    })
                                    .collect::<Vec<Message>>(),
                            ))
                            .ok();
                    }
                    Err(e) => {
                        resp_tx.send(NetworkResponse::Error(e)).ok();
                    }
                },
                NetworkRequest::RefreshToken => todo!(),
            }
        }
    }

    async fn fetch_messages(&self) -> Result<Vec<ServerMessage>, NetworkError> {
        let response: Response = self
            .client
            .get(format!("{}/messages", self.base_url))
            .send()
            .await?;
        response.error_for_status_ref()?;
        let messages = response
            .json::<Vec<ServerMessage>>()
            .await
            .map_err(|e| NetworkError::Deserialize(e))?;

        Ok(messages)
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
