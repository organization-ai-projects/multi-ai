use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct WebhookPayload {
    pub project: String,
    pub version: String,
    pub changes: Vec<String>,
    pub author: String,
    pub timestamp: u64,
}

pub struct WebhookManager {
    client: Client,
    discord_url: Option<String>,
    github_url: Option<String>,
}

impl WebhookManager {
    pub fn new(discord_url: Option<&str>, github_url: Option<&str>) -> Self {
        Self {
            client: reqwest::Client::new(),
            discord_url: discord_url.map(String::from),
            github_url: github_url.map(String::from),
        }
    }

    pub async fn notify_all(&self, payload: WebhookPayload) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(url) = &self.discord_url {
            self.notify_discord(url, &payload).await?;
        }
        if let Some(url) = &self.github_url {
            self.notify_github(url, &payload).await?;
        }
        Ok(())
    }

    async fn notify_discord(&self, url: &str, payload: &WebhookPayload) -> Result<(), reqwest::Error> {
        self.client.post(url)
            .json(&serde_json::json!({
                "content": format!("🚀 New version {} for {}", payload.version, payload.project),
                "embeds": [{
                    "title": "Changes",
                    "description": payload.changes.join("\n"),
                    "color": 5793266
                }]
            }))
            .send()
            .await?;
        Ok(())
    }

    async fn notify_github(&self, url: &str, payload: &WebhookPayload) -> Result<(), reqwest::Error> {
        // ...implementation similaire pour GitHub...
        Ok(())
    }
}
