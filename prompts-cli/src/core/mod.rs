use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct Prompts {
    storage: Box<dyn crate::storage::Storage + Send + Sync>,
}

impl Prompts {
    pub fn new(storage: Box<dyn crate::storage::Storage + Send + Sync>) -> Self {
        Self { storage }
    }

    pub async fn add_prompt(&self, prompt: &mut crate::storage::Prompt) -> Result<()> {
        self.storage.save_prompt(prompt).await
    }

    pub async fn list_prompts(&self) -> Result<Vec<crate::storage::Prompt>> {
        self.storage.load_prompts().await
    }

    pub async fn show_prompt(&self, query: &str) -> Result<Vec<crate::storage::Prompt>> {
        let prompts = self.storage.load_prompts().await?;
        let search_results = search_prompts(&prompts, query, &[], &[]);
        Ok(search_results)
    }

    pub async fn edit_prompt(&self, hash: &str, new_prompt: &mut crate::storage::Prompt) -> Result<()> {
        self.storage.delete_prompt(hash).await?;
        self.storage.save_prompt(new_prompt).await
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