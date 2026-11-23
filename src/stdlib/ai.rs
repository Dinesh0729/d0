#[derive(Debug, Clone)]
pub enum AIProvider {
    LocalOllama,
    OpenAICompatible,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct AIModelConfig {
    pub provider: AIProvider,
    pub model_name: String,
    pub api_endpoint: String,
}

#[derive(Debug, Clone)]
pub struct AICompletionRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

pub struct D0AI;

impl D0AI {
    pub fn create_config(provider: AIProvider, model_name: &str, endpoint: &str) -> AIModelConfig {
        AIModelConfig {
            provider,
            model_name: model_name.to_string(),
            api_endpoint: endpoint.to_string(),
        }
    }

    pub fn generate_stub(config: &AIModelConfig, prompt: &str) -> String {
        format!("[{}:{}] Response to: {}", config.api_endpoint, config.model_name, prompt)
    }
}
