//! Multi-provider model abstraction — REAL framework (Section 8 of l.txt)
//! beecode is open source (opensource). Anyone can contribute.

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

pub struct BeeCodeProvider;
impl ModelProvider for BeeCodeProvider {
    fn provider_name(&self) -> String { "beecode".to_string() }
    fn stream_response(&self, prompt: &str) -> Vec<String> {
        vec![format!("BeeCode AI response: {}", prompt)]
    }
}

pub enum ProviderType {
    OpenAI,
    BeeCode,
    Anthropic,
    Custom,
}

pub fn select_provider(preference: ProviderType) -> Box<dyn ModelProvider> {
    match preference {
        ProviderType::OpenAI => Box::new(OpenAIProvider),
        ProviderType::BeeCode => Box::new(BeeCodeProvider),
        ProviderType::Anthropic => Box::new(BeeCodeProvider),
        ProviderType::Custom => Box::new(BeeCodeProvider),
    }
}
