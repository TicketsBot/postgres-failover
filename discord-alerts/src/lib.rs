mod error;
pub use error::{Error, Result};

mod model;
use model::*;

mod colour;
pub use colour::*;

use std::str;

use log::error;

pub struct AlertClient {
    hostname: String,
    program_name: String,
    webhook_url: String,
    client: reqwest::Client,
}

impl AlertClient {
    #[cfg(feature = "auto-hostname")]
    pub fn new(program_name: &str, webhook_url: String) -> Result<Self> {
        let hostname = hostname::get()?
            .into_string()
            .unwrap_or_else(|_| "Unknown hostname".to_string());
        Ok(Self::new_with_hostname(hostname, program_name, webhook_url))
    }

    pub fn new_with_hostname(hostname: String, program_name: &str, webhook_url: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .use_rustls_tls()
            .build()
            .expect("failed to build http client");

        Self {
            hostname,
            program_name: program_name.to_string(),
            webhook_url,
            client,
        }
    }

    pub fn info(&self, message: &str) {
        self.spawn_send_message(Colour::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.spawn_send_message(Colour::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.spawn_send_message(Colour::Error, message);
    }

    fn spawn_send_message(&self, colour: Colour, message: &str) {
        let client = self.client.clone();
        let webhook_url = self.webhook_url.clone();
        let data = self.create_webhook_data(colour, message.to_string());

        tokio::spawn(async move {
           if let Err(e) = Self::send_message(client, webhook_url.as_str(), data).await {
               error!("Error sending Discord notification: {}", e);
           }
        });
    }

    async fn send_message(
        client: reqwest::Client,
        webhook_url: &str,
        data: WebhookBody,
    ) -> Result<()> {
        let res = client.post(webhook_url).json(&data).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let body = String::from_utf8_lossy(res.bytes().await?.as_ref()).to_string();
            Error::DiscordError(body).into()
        }
    }

    fn create_webhook_data(&self, colour: Colour, message: String) -> WebhookBody {
        WebhookBody {
            content: None,
            username: None,
            avatar_url: None,
            embeds: vec![Embed {
                title: Some(self.program_name.clone()),
                description: Some(message),
                url: None,
                color: Some(colour as u32),
                fields: vec![EmbedField {
                    name: "Hostname".to_string(),
                    value: self.hostname.clone(),
                    inline: false,
                }],
            }],
        }
    }
}
