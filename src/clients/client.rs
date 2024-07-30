use async_trait::async_trait;

pub struct LLMResponse {
    pub answer: String,
    pub reason: String,
}

#[async_trait]
pub trait Client: Send + Sync {
    async fn get_answer(
        &self,
        prompt: &str,
    ) -> Result<LLMResponse, Box<dyn std::error::Error + Send + Sync>>;
}
