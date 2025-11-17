use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::components::{Message, MessageSender};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Token {
    pub token: String,
    pub user_id: u32,
    pub expiry: u64,
    #[serde(skip)]
    pub username: Option<Arc<str>>,
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

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[allow(dead_code)]
pub enum NetworkRequest {
    Authenticate(AuthRequest),
    SendMessage {
        content: String,
        session: Arc<Token>,
    },
    FetchMessages,
    RefreshToken,
}

#[allow(dead_code)]
pub enum NetworkResponse {
    Auth(Arc<Token>),
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
    users_map: HashMap<u32, Arc<str>>,
}

impl NetworkTask {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: std::env::var("BASE_URL").unwrap_or(
                "http://ec2-44-250-68-143.us-west-2.compute.amazonaws.com:8000".to_string(),
            ),
            users_map: HashMap::new(),
        }
    }

    pub async fn run(
        &mut self,
        mut req_rx: mpsc::UnboundedReceiver<NetworkRequest>,
        resp_tx: mpsc::UnboundedSender<NetworkResponse>,
    ) {
        while let Some(req) = req_rx.recv().await {
            match req {
                NetworkRequest::Authenticate(auth_req) => match self.auth(&auth_req).await {
                    Ok(token) => {
                        resp_tx.send(NetworkResponse::Auth(token.clone())).ok();
                    }
                    Err(e) => {
                        resp_tx
                            .send(NetworkResponse::Error(NetworkError::Auth(e)))
                            .ok();
                    }
                },
                NetworkRequest::SendMessage { content, session } => {
                    match self.post_message(content, &session).await {
                        Ok(_) => {
                            resp_tx.send(NetworkResponse::MessageSent).ok();
                        }
                        Err(e) => {
                            resp_tx.send(NetworkResponse::Error(e)).ok();
                        }
                    }
                }
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
                                        username: self.users_map.get(&m.user_id).cloned(),
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

    async fn post_message(&self, content: String, session: &Token) -> Result<(), NetworkError> {
        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", session.token))
            .json(&json!(
                {
                    "body": content
                }
            ))
            .send()
            .await?;
        response.error_for_status_ref()?;

        Ok(())
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

    async fn auth(&mut self, auth_req: &AuthRequest) -> Result<Arc<Token>, AuthError> {
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

        if self.users_map.is_empty() {
            let response: Response = self
                .client
                .get(format!("{}/users", self.base_url))
                .send()
                .await?;
            response.error_for_status_ref()?;
            let users = response.json::<Vec<User>>().await?;
            self.users_map = users.into_iter().map(|u| (u.id, u.name.into())).collect()
        }

        let mut des_response = response.json::<Token>().await?;
        des_response.username = self.users_map.get(&des_response.user_id).cloned();
        Ok(des_response.into())
    }
}
