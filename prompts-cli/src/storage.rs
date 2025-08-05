use anyhow::Result;
use async_trait::async_trait;
use dirs;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use libsql::Connection;
use libsql::Builder;

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

#[async_trait]
pub trait Storage {
    async fn save_prompt(&self, prompt: &mut Prompt) -> Result<()>;
    async fn load_prompts(&self) -> Result<Vec<Prompt>>;
    async fn delete_prompt(&self, hash: &str) -> Result<()>;
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

#[async_trait]
impl Storage for JsonStorage {
    async fn save_prompt(&self, prompt: &mut Prompt) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", prompt.hash));
        let json = serde_json::to_string_pretty(prompt)?;
        tokio::fs::write(file_path, json).await?;
        Ok(())
    }

    async fn load_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();
        let mut read_dir = tokio::fs::read_dir(&self.storage_path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let json = tokio::fs::read_to_string(&path).await?;
                let prompt: Prompt = serde_json::from_str(&json)?;
                prompts.push(prompt);
            }
        }
        Ok(prompts)
    }

    async fn delete_prompt(&self, hash: &str) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", hash));
        if file_path.exists() {
            tokio::fs::remove_file(file_path).await?;
        }
        Ok(())
    }
}

pub struct LibSQLStorage {
    conn: Connection,
}

impl LibSQLStorage {
    pub async fn new(storage_path: Option<PathBuf>) -> Result<Self> {
        let db_path = match storage_path {
            Some(path) => path,
            None => {
                let mut default_path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
                default_path.push("prompts-cli");
                default_path.push("prompts.db");
                default_path
            }
        };

        let db = Builder::new_local(db_path.to_str().unwrap()).build().await?;
        let conn = db.connect()?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS prompts (
                hash TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                tags TEXT,
                categories TEXT
            )",
            (),
        ).await?;

        Ok(Self { conn })
    }
}

#[async_trait]
impl Storage for LibSQLStorage {
    async fn save_prompt(&self, prompt: &mut Prompt) -> Result<()> {
        let tags = prompt.tags.as_ref().map_or(Ok(None), |t| serde_json::to_string(t).map(Some))?.unwrap_or("[]".to_string());
        let categories = prompt.categories.as_ref().map_or(Ok(None), |c| serde_json::to_string(c).map(Some))?.unwrap_or("[]".to_string());

        self.conn.execute(
            "INSERT INTO prompts (hash, content, tags, categories) VALUES (?1, ?2, ?3, ?4)",
            libsql::params![prompt.hash.clone(), prompt.content.clone(), tags, categories],
        ).await?;

        Ok(())
    }

    async fn load_prompts(&self) -> Result<Vec<Prompt>> {
        todo!()
    }

    async fn delete_prompt(&self, hash: &str) -> Result<()> {
        todo!()
    }
}