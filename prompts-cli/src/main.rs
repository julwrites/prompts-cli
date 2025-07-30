use clap::Parser;
use prompts_cli::{
    search_prompts, GeneratorType, LLMTextGenerator, MockTextGenerator, Prompt, Storage,
    TextGenerator,
};
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the prompts storage directory
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Lists all the prompts
    List,
    /// Shows a specific prompt
    Show {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
    },

    /// Generates text based on a prompt
    Generate {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// Choose the text generation backend
        #[arg(long, value_enum, default_value_t = GeneratorType::Mock)]
        generator: GeneratorType,
    },
    /// Adds a new prompt
    Add {
        /// The text content of the prompt
        text: Option<String>,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Categories for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        categories: Vec<String>,
    },
    /// Edits an existing prompt
    Edit {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// The new text content of the prompt
        #[arg(short, long)]
        text: Option<String>,
        /// The new tags for the prompt (comma-separated)
        #[arg(short = 'a', long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// The new categories for the prompt (comma-separated)
        #[arg(short = 'e', long, value_delimiter = ',')]
        categories: Option<Vec<String>>,
    },
    /// Deletes a prompt
    Delete {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
    },
}

fn get_input(input: Option<String>, prompt_message: &str) -> anyhow::Result<String> {
    match input {
        Some(text) => Ok(text),
        None => {
            println!("{}", prompt_message);
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer.trim().to_string())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new(cli.config)?;

    match &cli.command {
        Commands::List => {
            let prompts = storage.load_prompts()?;
            for prompt in prompts {
                println!("{} - {}", &prompt.hash[..12], prompt.text);
            }
        }
        Commands::Show { query } => {
            let query = get_input(query.clone(), "Enter a query to search for a prompt:")?;
            let prompts = storage.load_prompts()?;
            let search_results = search_prompts(&prompts, &query, &[], &[]);

            if search_results.len() == 1 {
                println!("{}", search_results[0].text);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }

        Commands::Generate { query, generator } => {
            let query = get_input(query.clone(), "Enter a query to search for a prompt:")?;
            let prompts = storage.load_prompts()?;
            let search_results = search_prompts(&prompts, &query, &[], &[]);

            if search_results.len() == 1 {
                let prompt = search_results[0];
                let generated_text = match generator {
                    GeneratorType::Mock => {
                        let generator = MockTextGenerator;
                        generator.generate(&prompt.text).await
                    }
                    GeneratorType::Llm => {
                        let generator = LLMTextGenerator;
                        generator.generate(&prompt.text).await
                    }
                };
                println!("{}", generated_text);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }
        Commands::Add {
            text,
            tags,
            categories,
        } => {
            let text = get_input(text.clone(), "Enter the prompt text:")?;
            let mut prompt = Prompt {
                hash: "".to_string(),
                text,
                tags: tags.clone(),
                categories: categories.clone(),
            };
            storage.save_prompt(&mut prompt)?;
            println!("Prompt added successfully with hash: {}", &prompt.hash[..12]);
        }
        Commands::Edit {
            query,
            text,
            tags,
            categories,
        } => {
            let query = get_input(query.clone(), "Enter a query to find the prompt to edit:")?;
            let prompts = storage.load_prompts()?;
            let search_results = search_prompts(&prompts, &query, &[], &[]);

            if search_results.len() == 1 {
                let old_prompt = search_results[0];
                storage.delete_prompt(&old_prompt.hash)?;

                let mut new_prompt = Prompt {
                    hash: "".to_string(),
                    text: text.clone().unwrap_or_else(|| old_prompt.text.clone()),
                    tags: tags.clone().unwrap_or_else(|| old_prompt.tags.clone()),
                    categories: categories
                        .clone()
                        .unwrap_or_else(|| old_prompt.categories.clone()),
                };
                storage.save_prompt(&mut new_prompt)?;
                println!("Prompt {} updated to {}", &old_prompt.hash[..12], &new_prompt.hash[..12]);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }
        Commands::Delete { query } => {
            let query = get_input(query.clone(), "Enter a query to find the prompt to delete:")?;
            let prompts = storage.load_prompts()?;
            let search_results = search_prompts(&prompts, &query, &[], &[]);

            if search_results.len() == 1 {
                let prompt = search_results[0];
                storage.delete_prompt(&prompt.hash)?;
                println!("Prompt {} deleted successfully.", &prompt.hash[..12]);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }
    }

    Ok(())
}
