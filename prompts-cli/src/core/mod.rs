use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use sha2::{Digest, Sha256};

pub struct Prompts {
    storage: Box<dyn crate::storage::Storage + Send + Sync>,
}

impl Prompts {
    pub fn new(storage: Box<dyn crate::storage::Storage + Send + Sync>) -> Self {
        Self { storage }
    }

    pub async fn add_prompt(&self, prompt: &mut crate::storage::Prompt) -> Result<bool> {
        let prompts = self.storage.load_prompts().await?;
        if prompts.iter().any(|p| p.hash == prompt.hash) {
            return Ok(false);
        }
        self.storage.save_prompt(prompt).await?;
        Ok(true)
    }

    pub async fn list_prompts(&self, tags: Option<Vec<String>>) -> Result<Vec<crate::storage::Prompt>> {
        let prompts = self.storage.load_prompts().await?;
        if let Some(tags) = tags {
            let search_results = search_prompts(&prompts, "", &tags, &[]);
            Ok(search_results)
        } else {
            Ok(prompts)
        }
    }

    pub async fn show_prompt(&self, query: &str, tags: Option<Vec<String>>) -> Result<Vec<crate::storage::Prompt>> {
        let prompts = self.storage.load_prompts().await?;
        let search_results = search_prompts(&prompts, query, &tags.unwrap_or_default(), &[]);
        Ok(search_results)
    }

    pub async fn edit_prompt(
        &self,
        hash: &str,
        new_text: Option<String>,
        add_tags: Option<Vec<String>>,
        remove_tags: Option<Vec<String>>,
        add_categories: Option<Vec<String>>,
        remove_categories: Option<Vec<String>>,
    ) -> Result<()> {
        let mut prompts = self.storage.load_prompts().await?;
        let prompt_to_edit = prompts.iter_mut().find(|p| p.hash == hash);

        if let Some(prompt) = prompt_to_edit {
            if let Some(text) = new_text {
                prompt.content = text;
                let hash = Sha256::digest(prompt.content.as_bytes());
                prompt.hash = format!("{:x}", hash);
            }

            let mut tags = prompt.tags.clone().unwrap_or_default();
            if let Some(tags_to_add) = add_tags {
                tags.extend(tags_to_add);
                tags.sort();
                tags.dedup();
            }
            if let Some(tags_to_remove) = remove_tags {
                tags.retain(|t| !tags_to_remove.contains(t));
            }
            prompt.tags = Some(tags);

            let mut categories = prompt.categories.clone().unwrap_or_default();
            if let Some(categories_to_add) = add_categories {
                categories.extend(categories_to_add);
                categories.sort();
                categories.dedup();
            }
            if let Some(categories_to_remove) = remove_categories {
                categories.retain(|c| !categories_to_remove.contains(c));
            }
            prompt.categories = Some(categories);

            self.storage.delete_prompt(hash).await?;
            self.storage.save_prompt(prompt).await?;
        }

        Ok(())
    }

    pub async fn delete_prompt(&self, hash: &str) -> Result<()> {
        self.storage.delete_prompt(hash).await
    }
}

pub fn search_prompts(prompts: &[crate::storage::Prompt], query: &str, tags: &[String], categories: &[String]) -> Vec<crate::storage::Prompt> {
    let matcher = SkimMatcherV2::default();
    prompts.iter().filter(|p| {
        let content_match = query.is_empty() || matcher.fuzzy_match(&p.content, query).is_some();
        let tags_match = tags.is_empty() || p.tags.as_ref().map_or(false, |ptags| {
            tags.iter().all(|tag| ptags.contains(tag))
        });
        let categories_match = categories.is_empty() || p.categories.as_ref().map_or(false, |pcats| {
            categories.iter().all(|cat| pcats.contains(cat))
        });
        content_match && tags_match && categories_match
    }).cloned().collect()
}