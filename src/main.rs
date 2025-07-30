use clap::Parser;
use prompts_core::{
    GeneratorType, LLMTextGenerator, MockTextGenerator, Prompt, TextGenerator, load_prompts,
    save_prompts,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the prompts file
    #[arg(short, long)]
    file: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Lists all the prompts
    List,
    /// Shows a specific prompt
    Show {
        /// The name of the prompt to show
        name: String,
    },

    /// Generates text based on a prompt
    Generate {
        /// The name of the prompt to use for generation
        name: String,
        /// Choose the text generation backend
        #[arg(long, value_enum, default_value_t = GeneratorType::Mock)]
        generator: GeneratorType,
    },
    /// Adds a new prompt
    Add {
        /// The name of the prompt
        name: String,
        /// The text content of the prompt
        text: String,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Categories for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        categories: Vec<String>,
    },
    /// Searches for prompts by name, text, tags, or categories
    Search {
        /// The search query string
        #[arg(short, long)]
        query: Option<String>,
        /// Tags to filter by (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
        /// Categories to filter by (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        categories: Vec<String>,
    },
    /// Edits an existing prompt
    Edit {
        /// The name of the prompt to edit
        name: String,
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
        /// The name of the prompt to delete
        name: String,
    },
}

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            let prompts = load_prompts(&cli.file)?;
            for prompt in prompts {
                println!("{}: {}", prompt.name, prompt.text);
            }
        }
        Commands::Show { name } => {
            let prompts = load_prompts(&cli.file)?;
            if let Some(prompt) = prompts.iter().find(|p| p.name == *name) {
                println!("{}", prompt.text);
            } else {
                anyhow::bail!("Prompt '{}' not found", name);
            }
        }

        Commands::Generate {
            name,
            generator,
        } => {
            let prompts = load_prompts(&cli.file)?;
            if let Some(prompt) = prompts.iter().find(|p| p.name == *name) {
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
                anyhow::bail!("Prompt '{}' not found", name);
            }
        }
        Commands::Add {
            name,
            text,
            tags,
            categories,
        } => {
            let mut prompts = load_prompts(&cli.file).unwrap_or_else(|_| Vec::new());
            if prompts.iter().any(|p| p.name == *name) {
                anyhow::bail!("Prompt '{}' already exists", name);
            }
            prompts.push(Prompt {
                name: name.clone(),
                text: text.clone(),
                tags: tags.clone(),
                categories: categories.clone(),
            });
            save_prompts(&cli.file, &prompts)?;
            println!("Prompt '{}' added successfully.", name);
        }
        Commands::Search {
            query,
            tags,
            categories,
        } => {
            let prompts = load_prompts(&cli.file)?;
            let search_results = prompts_core::search_prompts(
                &prompts,
                query.as_deref().unwrap_or(""),
                &tags,
                &categories,
            );

            if search_results.is_empty() {
                println!("No prompts found matching your criteria.");
            } else {
                for prompt in search_results {
                    println!(
                        "Name: {}\nText: {}\nTags: {:?}\nCategories: {:?}\n---",
                        prompt.name, prompt.text, prompt.tags, prompt.categories
                    );
                }
            }
        }
        Commands::Edit {
            name,
            text,
            tags,
            categories,
        } => {
            let mut prompts = load_prompts(&cli.file)?;
            if let Some(prompt) = prompts.iter_mut().find(|p| p.name == *name) {
                if let Some(new_text) = text {
                    prompt.text = new_text.clone();
                }
                if let Some(new_tags) = tags {
                    prompt.tags = new_tags.clone();
                }
                if let Some(new_categories) = categories {
                    prompt.categories = new_categories.clone();
                }
                save_prompts(&cli.file, &prompts)?;
                println!("Prompt '{}' updated successfully.", name);
            } else {
                anyhow::bail!("Prompt '{}' not found", name);
            }
        }
        Commands::Delete { name } => {
            let mut prompts = load_prompts(&cli.file)?;
            let initial_len = prompts.len();
            prompts.retain(|p| p.name != *name);
            if prompts.len() < initial_len {
                save_prompts(&cli.file, &prompts)?;
                println!("Prompt '{}' deleted successfully.", name);
            } else {
                anyhow::bail!("Prompt '{}' not found", name);
            }
        }
    }

    Ok(())
}
