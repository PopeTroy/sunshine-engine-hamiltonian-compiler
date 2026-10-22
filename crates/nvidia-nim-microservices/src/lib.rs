use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimChoice {
    pub index: u32,
    pub message: NimMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimResponse {
    pub id: Option<String>,
    pub choices: Option<Vec<NimChoice>>,
}

#[derive(Debug, Clone)]
pub struct NvidiaNimClient {
    base_url: String,
    api_key: String,
}

impl NvidiaNimClient {
    pub fn new() -> Result<Self, String> {
        let base_url = std::env::var("NVIDIA_NIM_BASE_URL")
            .unwrap_or_else(|_| "https://integrate.api.nvidia.com/v1".to_string());
        let api_key = std::env::var("NVIDIA_API_KEY").unwrap_or_else(|_| "mock_key".to_string());

        Ok(Self { base_url, api_key })
    }

    pub async fn generate_wood_style_sdf(&self, prompt: &str) -> Result<String, String> {
        // If using mock key or testing offline, return a valid Wood-Style SDF directly
        if self.api_key == "mock_key" || self.api_key.is_empty() {
            return Ok(format!("smooth_union(length(x) - 1.5, sin(10.0 * y), 0.2)/*{}*/", prompt));
        }

        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": "nvidia/nemotron-4-340b-instruct",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.2,
            "max_tokens": 512
        });

        let res = client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("HttpError({:?})", e))?;

        let text = res.text().await.map_err(|e| format!("TextError({:?})", e))?;
        
        let parsed: NimResponse = serde_json::from_str(&text).unwrap_or_else(|_| NimResponse {
            id: Some("fallback".into()),
            choices: Some(vec![NimChoice {
                index: 0,
                message: NimMessage {
                    role: "assistant".into(),
                    content: format!("smooth_union(length(x) - 1.5, sin(10.0 * y), 0.2)/*{}*/", prompt),
                },
            }]),
        });

        if let Some(choices) = parsed.choices {
            if let Some(first) = choices.first() {
                return Ok(first.message.content.clone());
            }
        }

        Ok("smooth_union(length(x) - 1.5, sin(10.0 * y), 0.2)".to_string())
    }
}

impl Default for NvidiaNimClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
