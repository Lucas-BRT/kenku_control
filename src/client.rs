use async_trait::async_trait;
use mockall::automock;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{error::Error, time::Duration};

use crate::DEFAULT_KENKU_REMOTE_ADDRESS;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

#[automock]
#[async_trait]
pub trait HttpClient {
    async fn ping(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get<T>(&self, url: &str) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static;

    async fn post<T>(&self, url: &str, body: &Value) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static;

    async fn put<T>(&self, url: &str, body: &Value) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static;
}

pub struct ReqwestClient(Client);

impl ReqwestClient {
    pub fn new() -> Self {
        Self(Client::new())
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Client> for ReqwestClient {
    fn from(value: Client) -> Self {
        Self(value)
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn ping(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = Url::parse(&DEFAULT_KENKU_REMOTE_ADDRESS.to_string())?;
        println!("Pinging {}", url);
        self.0.get(url).timeout(DEFAULT_TIMEOUT).send().await?;
        Ok(())
    }

    async fn get<T>(&self, url: &str) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let response = self
            .0
            .get(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Ok(response.json().await?)
    }

    async fn post<T>(&self, url: &str, body: &Value) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let response = self
            .0
            .post(url)
            .json(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Ok(response.json().await?)
    }

    async fn put<T>(&self, url: &str, body: &Value) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let response = self
            .0
            .put(url)
            .json(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_get() {
        let mut client = MockHttpClient::new();
        client
            .expect_get::<Value>()
            .with(eq("https://example.com"))
            .returning(|_| Ok(json!({})));
    }

    #[tokio::test]
    async fn test_post() {
        let mut client = MockHttpClient::new();
        client
            .expect_post()
            .with(eq("https://example.com"), eq(json!({})))
            .returning(|_, _| Ok(json!({})));
    }

    #[tokio::test]
    async fn test_put() {
        let mut client = MockHttpClient::new();
        client
            .expect_put()
            .with(eq("https://example.com"), eq(json!({})))
            .returning(|_, _| Ok(json!({})));
    }
}
