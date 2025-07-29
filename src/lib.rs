use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use async_trait::async_trait;
use clap::ValueEnum;

#[derive(Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub name: String,
    pub text: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum GeneratorType {
    /// Use a mock text generator
    Mock,
    /// Use a large language model (LLM) text generator
    Llm,
}

#[async_trait]
pub trait TextGenerator {
    async fn generate(&self, prompt_text: &str) -> String;
}

pub struct MockTextGenerator;

#[async_trait]
impl TextGenerator for MockTextGenerator {
    async fn generate(&self, prompt_text: &str) -> String {
        format!("Generated text for '{}': {}\n(This is a mock generation)", prompt_text, prompt_text)
    }
}

pub struct LLMTextGenerator;

#[async_trait]
impl TextGenerator for LLMTextGenerator {
    async fn generate(&self, prompt_text: &str) -> String {
        format!("Generated text for '{}' using LLM: {}\n(This is a placeholder for LLM API call)", prompt_text, prompt_text)
    }
}

pub fn load_prompts(file_path: &str) -> Result<Vec<Prompt>, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let prompts: Vec<Prompt> = serde_json::from_str(&data)?;
    Ok(prompts)
}
