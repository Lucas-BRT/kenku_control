use crate::DEFAULT_KENKU_REMOTE_ADDRESS;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{error::Error, future::Future, time::Duration};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub trait HttpClient {
    fn ping(&self) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync>>>;
    fn get<T>(&self, url: &str) -> impl Future<Output = Result<T, Box<dyn Error + Send + Sync>>>
    where
        T: DeserializeOwned + Send + 'static;

    fn post<T>(
        &self,
        url: &str,
        body: &Value,
    ) -> impl Future<Output = Result<T, Box<dyn Error + Send + Sync>>>
    where
        T: DeserializeOwned + Send + 'static;

    fn put<T>(
        &self,
        url: &str,
        body: &Value,
    ) -> impl Future<Output = Result<T, Box<dyn Error + Send + Sync>>>
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
