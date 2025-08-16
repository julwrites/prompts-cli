use clap::Parser;
use prompts_cli::{AppError, Prompt, Prompts, JsonStorage, LibSQLStorage, Storage};
use std::io::{self, Read};
use std::path::PathBuf;
use config::{Config, File, FileFormat};

#[derive(Debug, serde::Deserialize)]
struct AppConfig {
    storage: StorageConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            storage: StorageConfig::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct StorageConfig {
    #[serde(default = "default_storage_type")]
    r#type: String,
    path: Option<PathBuf>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            r#type: default_storage_type(),
            path: None,
        }
    }
}

fn default_storage_type() -> String {
    "json".to_string()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the prompts storage directory
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// The output format
    #[arg(long)]
    output: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Lists all the prompts
    List {
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// Shows a specific prompt
    Show {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Generates text based on a prompt
    Generate {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Variables to use for templating (key=value pairs)
        #[arg(short, long, value_parser = parse_key_val, action = clap::ArgAction::Append)]
        variables: Vec<(String, String)>,
    },
    /// Adds a new prompt
    Add {
        /// The text content of the prompt
        text: Option<String>,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Categories for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        categories: Option<Vec<String>>,
    },
    /// Edits an existing prompt
    Edit {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// Tags for filtering (comma-separated)
        #[arg(long, value_delimiter = ',')]
        filter_tags: Option<Vec<String>>,
        /// The new text content of the prompt
        #[arg(short, long)]
        text: Option<String>,
        /// Tags to add to the prompt (comma-separated)
        #[arg(long, value_delimiter = ',')]
        add_tags: Option<Vec<String>>,
        /// Tags to remove from the prompt (comma-separated)
        #[arg(long, value_delimiter = ',')]
        remove_tags: Option<Vec<String>>,
        /// Categories to add to the prompt (comma-separated)
        #[arg(long, value_delimiter = ',')]
        add_categories: Option<Vec<String>>,
        /// Categories to remove from the prompt (comma-separated)
        #[arg(long, value_delimiter = ',')]
        remove_categories: Option<Vec<String>>,
    },
    /// Deletes a prompt
    Delete {
        /// The fuzzy query to search for a prompt
        query: Option<String>,
        /// Tags for the prompt (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// Imports prompts from a directory
    Import {
        /// The directory to import prompts from
        path: PathBuf,
    },
    /// Exports prompts to a directory
    Export {
        /// The directory to export prompts to
        path: PathBuf,
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

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=VALUE: no '=' found in `{}`", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

async fn run_cli(cli: Cli) -> Result<(), AppError> {
    let app_config: AppConfig = if let Some(config_path) = &cli.config {
        Config::builder()
            .add_source(File::new(config_path.to_str().unwrap(), FileFormat::Toml))
            .build()?.try_deserialize()?
    } else {
        let config_builder = match std::env::var("PROMPTS_CLI_CONFIG_PATH") {
            Ok(path) => Config::builder().add_source(File::new(&path, FileFormat::Toml).required(true)),
            Err(_) => {
                let config_path = match std::env::var("PROMPTS_CLI_CONFIG_DIR_FOR_TESTING") {
                    Ok(path) => PathBuf::from(path).join("prompts-cli/config.toml"),
                    Err(_) => dirs::config_dir()
                        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                        .join("prompts-cli/config.toml"),
                };
                Config::builder()
                    .add_source(File::new(config_path.to_str().unwrap(), FileFormat::Toml).required(false))
            }
        };
        config_builder.build()?.try_deserialize().unwrap_or_default()
    };

    let storage_path = app_config.storage.path;

    let storage: Box<dyn Storage + Send + Sync> = match app_config.storage.r#type.as_str() {
        "json" => Box::new(JsonStorage::new(storage_path)?),
        "libsql" => Box::new(LibSQLStorage::new(storage_path).await?),
        _ => return Err(AppError::Storage("Invalid storage type".to_string())),
    };

    let prompts_api = Prompts::new(storage);

    match &cli.command {
        Commands::List { tags } => {
            let prompts = prompts_api.list_prompts(tags.clone()).await?;
            for prompt in prompts {
                println!("{} - {}", &prompt.hash[..12], prompt.content);
            }
        }
        Commands::Show { query, tags } => {
            let query_str = get_input(query.clone(), "Enter a query to search for a prompt:")?;
            let search_results = prompts_api.show_prompt(&query_str, tags.clone()).await?;

            if search_results.len() == 1 {
                println!("{}", search_results[0].content);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }

        Commands::Generate { query, tags, variables } => {
            let query_str = get_input(query.clone(), "Enter a query to search for a prompt:")?;
            let search_results = prompts_api.show_prompt(&query_str, tags.clone()).await?;

            if search_results.len() == 1 {
                let prompt = &search_results[0];
                let mut context = tera::Context::new();
                for (key, value) in variables {
                    context.insert(key, &value);
                }
                let rendered_prompt = tera::Tera::one_off(&prompt.content, &context, false)
                    .map_err(|e| AppError::Anyhow(e.to_string()))?;
                println!("{}", rendered_prompt);
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
            let text_content = get_input(text.clone(), "Enter the prompt text:")?;
            let mut prompt = Prompt::new(&text_content, tags.clone(), categories.clone());
            if prompts_api.add_prompt(&mut prompt).await? {
                println!("Prompt added successfully with hash: {}", &prompt.hash[..12]);
            } else {
                println!("Prompt already exists.");
            }
        }
        Commands::Edit {
            query,
            filter_tags,
            text,
            add_tags,
            remove_tags,
            add_categories,
            remove_categories,
        } => {
            let query_str = get_input(query.clone(), "Enter a query to find the prompt to edit:")?;
            let search_results = prompts_api.show_prompt(&query_str, filter_tags.clone()).await?;

            if search_results.len() == 1 {
                let old_prompt = &search_results[0];
                let old_prompt_hash = old_prompt.hash.clone();

                prompts_api.edit_prompt(
                    &old_prompt_hash,
                    text.clone(),
                    add_tags.clone(),
                    remove_tags.clone(),
                    add_categories.clone(),
                    remove_categories.clone(),
                ).await?;

                println!("Prompt {} updated.", &old_prompt_hash[..12]);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }
        Commands::Delete { query, tags } => {
            let query_str = get_input(query.clone(), "Enter a query to find the prompt to delete:")?;
            let search_results = prompts_api.show_prompt(&query_str, tags.clone()).await?;

            if search_results.len() == 1 {
                let prompt_hash = search_results[0].hash.clone();
                prompts_api.delete_prompt(&prompt_hash).await?;
                println!("Prompt {} deleted successfully.", &prompt_hash[..12]);
            } else {
                let result_json = serde_json::to_string_pretty(&search_results)?;
                println!("{}", result_json);
            }
        }
        Commands::Import { path } => {
            let mut imported_count = 0;
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    let json = std::fs::read_to_string(&path)?;
                    let mut prompt: Prompt = serde_json::from_str(&json)?;
                    prompts_api.add_prompt(&mut prompt).await?;
                    imported_count += 1;
                }
            }
            println!("Imported {} prompts.", imported_count);
        }
        Commands::Export { path } => {
            std::fs::create_dir_all(path)?;
            let prompts = prompts_api.list_prompts(None).await?;
            let mut exported_count = 0;
            for prompt in prompts {
                let file_path = path.join(format!("{}.json", prompt.hash));
                let json = serde_json::to_string_pretty(&prompt)?;
                std::fs::write(file_path, json)?;
                exported_count += 1;
            }
            println!("Exported {} prompts.", exported_count);
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let output_json = cli.output.as_deref() == Some("json");

    if let Err(e) = run_cli(cli).await {
        if output_json {
            if let Ok(json) = serde_json::to_string(&e) {
                println!("{}", json);
            } else {
                // Fallback for serialization errors
                println!("{{\"error\":\"Failed to serialize error message\"}}");
            }
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}
