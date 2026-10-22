use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NimError {
    #[error("NVIDIA API Key missing.")]
    MissingApiKey,
    #[error("HTTP Transport Error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("NIM Response Parse Error: {0}")]
    ParseError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct NemotronRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct NemotronChoice {
    pub message: ChatMessage,
}

#[derive(Deserialize, Debug)]
pub struct NemotronResponse {
    pub choices: Vec<NemotronChoice>,
}

pub struct NvidiaNimClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl NvidiaNimClient {
    pub fn new() -> Result<Self, NimError> {
        let api_key = std::env::var("NVIDIA_API_KEY").unwrap_or_else(|_| "mock-nvapi-key".to_string());
        let base_url = std::env::var("NVIDIA_NIM_BASE_URL")
            .unwrap_or_else(|_| "https://integrate.api.nvidia.com/v1".to_string());

        Ok(Self {
            api_key,
            base_url,
            client: reqwest::Client::new(),
        })
    }

    pub async fn generate_wood_style_sdf(&self, user_prompt: &str) -> Result<String, NimError> {
        if self.api_key == "mock-nvapi-key" {
            return Ok(format!(
                "sdf::wood_style_growth(branch_count=12, geometry=\"{}\")",
                user_prompt
            ));
        }

        let system_prompt = "You are the Sunshine Engine Xeno-IR Wood-Style Compiler Assistant. Convert prompts into valid L-System Wood SDFs using Nemotron-4 340B.";
        let request = NemotronRequest {
            model: "nvidia/nemotron-4-340b-instruct".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.2,
            max_tokens: 512,
        };

        let res = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let body: NemotronResponse = res.json().await?;
        body.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| NimError::ParseError("Empty response from Nemotron NIM".to_string()))
    }
}
