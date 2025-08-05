use anyhow::Result;
use async_trait::async_trait;
use dirs;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use libsql::Connection;
use libsql::Builder;
use std::fs;

/// Represents a prompt with its content, metadata, and a unique hash.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Prompt {
    /// The text content of the prompt.
    pub content: String,
    /// Optional tags associated with the prompt.
    pub tags: Option<Vec<String>>,
    /// Optional categories for grouping prompts.
    pub categories: Option<Vec<String>>,
    /// A unique SHA256 hash of the prompt's content, used for identification.
    pub hash: String,
}

impl Prompt {
    /// Creates a new `Prompt` instance.
    ///
    /// The `hash` is automatically generated from the content.
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

/// A trait defining the interface for prompt storage.
#[async_trait]
pub trait Storage {
    /// Saves a prompt to the storage.
    async fn save_prompt(&self, prompt: &mut Prompt) -> Result<()>;
    /// Loads all prompts from the storage.
    async fn load_prompts(&self) -> Result<Vec<Prompt>>;
    /// Deletes a prompt from the storage by its hash.
    async fn delete_prompt(&self, hash: &str) -> Result<()>;
}

/// A storage implementation that uses JSON files.
///
/// Each prompt is stored as a separate JSON file in a specified directory.
pub struct JsonStorage {
    storage_path: PathBuf,
}

/// Returns the default storage directory for the application.
///
/// This is typically `~/.config/prompts-cli`.
fn get_default_storage_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    path.push("prompts-cli");
    Ok(path)
}

impl JsonStorage {
    /// Creates a new `JsonStorage` instance.
    ///
    /// If `storage_path` is `None`, a default directory is used.
    pub fn new(storage_path: Option<PathBuf>) -> Result<Self> {
        let path = match storage_path {
            Some(path) => path,
            None => {
                let mut default_path = get_default_storage_dir()?;
                default_path.push("prompts");
                fs::create_dir_all(&default_path)?;
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

/// A storage implementation that uses a LibSQL database.
///
/// All prompts are stored in a single database file.
pub struct LibSQLStorage {
    conn: Connection,
}

impl LibSQLStorage {
    /// Creates a new `LibSQLStorage` instance.
    ///
    /// If `storage_path` is `None`, a default database file is used.
    /// This will also create the necessary tables if they don't exist.
    pub async fn new(storage_path: Option<PathBuf>) -> Result<Self> {
        let db_path = match storage_path {
            Some(path) => path,
            None => {
                let mut path = get_default_storage_dir()?;
                fs::create_dir_all(&path)?;
                path.push("prompts.db");
                path
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
        let tags = serde_json::to_string(&prompt.tags.as_deref().unwrap_or_default())?;
        let categories = serde_json::to_string(&prompt.categories.as_deref().unwrap_or_default())?;

        self.conn.execute(
            "INSERT INTO prompts (hash, content, tags, categories) VALUES (?1, ?2, ?3, ?4)",
            libsql::params![prompt.hash.clone(), prompt.content.clone(), tags, categories],
        ).await?;

        Ok(())
    }

    async fn load_prompts(&self) -> Result<Vec<Prompt>> {
        let mut rows = self.conn.query("SELECT hash, content, tags, categories FROM prompts", ()).await?;
        let mut prompts = Vec::new();

        while let Some(row) = rows.next().await? {
            let hash: String = row.get(0)?;
            let content: String = row.get(1)?;
            let tags_str: String = row.get(2)?;
            let categories_str: String = row.get(3)?;

            let tags: Option<Vec<String>> = serde_json::from_str(&tags_str)?;
            let categories: Option<Vec<String>> = serde_json::from_str(&categories_str)?;

            prompts.push(Prompt {
                hash,
                content,
                tags,
                categories,
            });
        }

        Ok(prompts)
    }

    async fn delete_prompt(&self, hash: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM prompts WHERE hash = ?1",
            libsql::params![hash],
        ).await?;
        Ok(())
    }
}