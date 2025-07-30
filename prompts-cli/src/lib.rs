use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use async_trait::async_trait;
use clap::ValueEnum;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Prompt {
    pub name: String,
    pub text: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
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

pub fn save_prompts(file_path: &str, prompts: &[Prompt]) -> Result<(), io::Error> {
    let data = serde_json::to_string_pretty(prompts)?;
    fs::write(file_path, data)?;
    Ok(())
}

pub fn search_prompts(prompts: &[Prompt], query: &str, tags: &[String], categories: &[String]) -> Vec<Prompt> {
    let query_lower = query.to_lowercase();
    prompts.iter().filter(|p| {
        let name_lower = p.name.to_lowercase();
        let text_lower = p.text.to_lowercase();

        let matches_query = query.is_empty() || name_lower.contains(&query_lower) || text_lower.contains(&query_lower);
        let matches_tags = tags.is_empty() || tags.iter().all(|t| p.tags.contains(t));
        let matches_categories = categories.is_empty() || categories.iter().all(|c| p.categories.contains(c));

        matches_query && matches_tags && matches_categories
    }).cloned().collect()
}