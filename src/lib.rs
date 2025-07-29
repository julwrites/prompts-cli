use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct Prompt {
    pub name: String,
    pub text: String,
}

pub fn load_prompts(file_path: &str) -> Result<Vec<Prompt>, io::Error> {
    let data = fs::read_to_string(file_path)?;
    let prompts: Vec<Prompt> = serde_json::from_str(&data)?;
    Ok(prompts)
}
