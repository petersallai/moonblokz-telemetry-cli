use anyhow::{anyhow, Context, Result};
use reqwest::StatusCode;

use crate::config::Config;
use crate::parser::Command;

pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            config,
            http_client,
        })
    }
    
    pub async fn send_command(&self, command: &Command) -> Result<String> {
        let json_payload = command.to_json()?;
        
        let url = format!("{}/command", self.config.hub_url);
        
        let response = self.http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Api-Key", &self.config.api_key)
            .json(&json_payload)
            .send()
            .await
            .context("Failed to send request to hub")?;
        
        let status = response.status();
        
        match status {
            StatusCode::OK => Ok("OK".to_string()),
            StatusCode::UNAUTHORIZED => {
                Err(anyhow!("Command error: 401 Unauthorized - Invalid API key"))
            }
            status if status.is_client_error() => {
                let body = response.text().await.unwrap_or_default();
                Err(anyhow!("Command error: {} - {}", status.as_u16(), body))
            }
            status if status.is_server_error() => {
                let body = response.text().await.unwrap_or_default();
                Err(anyhow!("Server error: {} - {}", status.as_u16(), body))
            }
            _ => {
                Err(anyhow!("Unexpected response: {}", status.as_u16()))
            }
        }
    }
}
