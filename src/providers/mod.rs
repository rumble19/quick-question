pub mod claude;

use anyhow::Result;

#[async_trait::async_trait]
pub trait Provider {
    async fn ask(&self, question: &str) -> Result<String>;
}
