use anyhow::Result;
use dirs;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub hash: String,
}

impl Prompt {
    pub fn new(content: &str, tags: Option<Vec<String>>, categories: Option<Vec<String>>) -> Self {
        let hash = Sha256::digest(content.as_bytes());
        Self {
            content: content.to_string(),
            tags,
            categories,
            hash: format!("{:x}", hash),
        }
    }
}

pub trait Storage {
    fn save_prompt(&self, prompt: &mut Prompt) -> Result<()>;
    fn load_prompts(&self) -> Result<Vec<Prompt>>;
    fn delete_prompt(&self, hash: &str) -> Result<()>;
}

pub struct JsonStorage {
    storage_path: PathBuf,
}

impl JsonStorage {
    pub fn new(storage_path: Option<PathBuf>) -> Result<Self> {
        let path = match storage_path {
            Some(path) => path,
            None => {
                let mut default_path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
                default_path.push("prompts-cli");
                default_path.push("prompts");
                std::fs::create_dir_all(&default_path)?;
                default_path
            }
        };
        Ok(Self { storage_path: path })
    }
}

impl Storage for JsonStorage {
    fn save_prompt(&self, prompt: &mut Prompt) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", prompt.hash));
        let json = serde_json::to_string_pretty(prompt)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }

    fn load_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();
        for entry in std::fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let json = std::fs::read_to_string(&path)?;
                let prompt: Prompt = serde_json::from_str(&json)?;
                prompts.push(prompt);
            }
        }
        Ok(prompts)
    }

    fn delete_prompt(&self, hash: &str) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", hash));
        if file_path.exists() {
            std::fs::remove_file(file_path)?;
        }
        Ok(())
    }
}