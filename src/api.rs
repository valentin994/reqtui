use std::{error::Error, fmt, time::Duration};

use reqwest::{Client, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    #[default]
    GET,
    POST,
    DELETE,
    PATCH,
    PUT,
}

impl RequestType {
    const VARIANTS: [RequestType; 5] = [
        RequestType::GET,
        RequestType::POST,
        RequestType::DELETE,
        RequestType::PATCH,
        RequestType::PUT,
    ];
    pub fn next(self) -> RequestType {
        let idx = RequestType::VARIANTS
            .iter()
            .position(|&r| r == self)
            .unwrap();
        RequestType::VARIANTS[(idx + 1) % RequestType::VARIANTS.len()]
    }
}

#[derive(Debug, Hash, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Protocol {
    #[default]
    HTTP,
    HTTPS,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::HTTP => write!(f, "http"),
            Protocol::HTTPS => write!(f, "https"),
        }
    }
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub protocol: Protocol,
    pub request_type: RequestType,
    pub url: String,
    pub body: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}| {}", self.request_type, self.name)
    }
}

impl Request {
    pub async fn send(&self, client: &Client) -> Result<String, Box<dyn Error>> {
        let prepare_request = match self.request_type {
            RequestType::GET => client.get(format!("{}://{}", self.protocol, self.url)),
            RequestType::POST => client
                .post(format!("{}://{}", self.protocol, self.url))
                .body(self.body.clone())
                .header(CONTENT_TYPE, "application/json"),
            RequestType::PUT => client
                .put(format!("{}://{}", self.protocol, self.url))
                .body(self.body.clone())
                .header(CONTENT_TYPE, "application/json"),
            RequestType::PATCH => client
                .patch(format!("{}://{}", self.protocol, self.url))
                .body(self.body.clone())
                .header(CONTENT_TYPE, "application/json"),
            RequestType::DELETE => client.delete(format!("{}://{}", self.protocol, self.url)),
        };
        // TODO: use config timeout
        match prepare_request.timeout(Duration::from_secs(4)).send().await {
            Ok(resp) => Ok(resp.text().await?),
            Err(e) if e.is_timeout() => Ok("timeout".to_string()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
