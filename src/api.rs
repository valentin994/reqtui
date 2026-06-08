use std::{error::Error, fmt};

use reqwest::Client;

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    #[default]
    GET,
    POST,
    DELETE,
    PATCH,
    PUT,
}

#[derive(Debug, Hash, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Eq, Debug, PartialEq)]
pub struct Request {
    pub protocol: Protocol,
    pub request_type: RequestType,
    pub url: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}{}", self.request_type, self.protocol, self.url)
    }
}

impl Request {
    pub async fn send(&self, client: &Client) -> Result<String, Box<dyn Error>> {
        let prepare_request = match self.request_type {
            RequestType::GET => client.get(format!("{}://{}", self.protocol, self.url)),
            RequestType::POST => client.post(format!("{}://{}", self.protocol, self.url)),
            RequestType::PUT => client.put(format!("{}://{}", self.protocol, self.url)),
            RequestType::PATCH => client.patch(format!("{}://{}", self.protocol, self.url)),
            RequestType::DELETE => client.delete(format!("{}://{}", self.protocol, self.url)),
        };
        let req = prepare_request.send().await?;
        Ok(req.text().await?)
    }
}
