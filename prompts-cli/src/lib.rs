use anyhow::{Context, Result};
use async_trait::async_trait;
use clap::ValueEnum;
use directories::ProjectDirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Prompt {
    #[serde(skip_deserializing)]
    pub hash: String,
    pub text: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum GeneratorType {
    Mock,
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
        format!(
            "Generated text for '{}': {}\n(This is a mock generation)",
            prompt_text, prompt_text
        )
    }
}

pub struct LLMTextGenerator;

#[async_trait]
impl TextGenerator for LLMTextGenerator {
    async fn generate(&self, prompt_text: &str) -> String {
        format!(
            "Generated text for '{}' using LLM: {}\n(This is a placeholder for LLM API call)",
            prompt_text, prompt_text
        )
    }
}

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let base_path = match path {
            Some(p) => p,
            None => {
                let proj_dirs = ProjectDirs::from("com", "example", "prompts-cli")
                    .context("Failed to get project directories")?;
                proj_dirs.data_dir().to_path_buf()
            }
        };

        let prompts_path = base_path.join("prompts");

        if !prompts_path.exists() {
            fs::create_dir_all(&prompts_path).context("Failed to create prompts directory")?;
        }

        Ok(Self { path: prompts_path })
    }

    pub fn save_prompt(&self, prompt: &mut Prompt) -> Result<()> {
        let hash = Self::calculate_hash(&prompt.text);
        prompt.hash = hash.clone();
        let file_path = self.path.join(format!("{}.json", hash));

        if !file_path.exists() {
            let data = serde_json::to_string_pretty(prompt)?;
            fs::write(file_path, data)?;
        }

        Ok(())
    }

    pub fn load_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path)?;
                let mut prompt: Prompt = serde_json::from_str(&data)?;
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    prompt.hash = stem.to_string();
                }
                prompts.push(prompt);
            }
        }
        Ok(prompts)
    }

    pub fn delete_prompt(&self, hash: &str) -> Result<()> {
        let file_path = self.path.join(format!("{}.json", hash));
        if file_path.exists() {
            fs::remove_file(file_path).context("Failed to delete prompt file")?;
        }
        Ok(())
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    fn calculate_hash(text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text);
        format!("{:x}", hasher.finalize())
    }
}

pub fn search_prompts<'a>(
    prompts: &'a [Prompt],
    query: &str,
    tags: &[String],
    categories: &[String],
) -> Vec<&'a Prompt> {
    let matcher = SkimMatcherV2::default();

    prompts
        .iter()
        .filter(|p| {
            let matches_query = query.is_empty() || matcher.fuzzy_match(&p.text, query).is_some();
            let matches_tags = tags.is_empty() || tags.iter().all(|t| p.tags.contains(t));
            let matches_categories =
                categories.is_empty() || categories.iter().all(|c| p.categories.contains(c));

            matches_query && matches_tags && matches_categories
        })
        .collect()
}