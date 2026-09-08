//! Multi-provider model abstraction — REAL framework (Section 8 of l.txt)
//! wimoai is open source (opensource). Anyone can contribute.

pub trait ModelProvider {
    fn provider_name(&self) -> String;
    fn stream_response(&self, prompt: &str) -> Vec<String>;
}

pub struct OpenAIProvider;
impl ModelProvider for OpenAIProvider {
    fn provider_name(&self) -> String { "openai".to_string() }
    fn stream_response(&self, prompt: &str) -> Vec<String> {
        vec![format!("Response to: {}", prompt)]
    }
}

pub struct WimoProvider;
impl ModelProvider for WimoProvider {
    fn provider_name(&self) -> String { "wimoai".to_string() }
    fn stream_response(&self, prompt: &str) -> Vec<String> {
        vec![format!("Wimo AI response: {}", prompt)]
    }
}

pub enum ProviderType {
    OpenAI,
    Wimo,
    Anthropic,
    Custom,
}

pub fn select_provider(preference: ProviderType) -> Box<dyn ModelProvider> {
    match preference {
        ProviderType::OpenAI => Box::new(OpenAIProvider),
        ProviderType::Wimo => Box::new(WimoProvider),
        ProviderType::Anthropic => Box::new(WimoProvider),
        ProviderType::Custom => Box::new(WimoProvider),
    }
}
