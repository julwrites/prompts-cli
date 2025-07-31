use clap::Parser;
use prompts_cli::Prompt;
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

    match &cli.command {
        Commands::List => {
            println!("List command");
        }
        Commands::Show { query } => {
            println!("Show command");
        }

        Commands::Generate { query } => {
            println!("Generate command");
        }
        Commands::Add {
            text,
            tags,
            categories,
        } => {
            let text = get_input(text.clone(), "Enter the prompt text:")?;
            let _prompt = Prompt::new(&text, tags.clone(), categories.clone());
            println!("Add command");
        }
        Commands::Edit {
            query,
            text,
            tags,
            categories,
        } => {
            println!("Edit command");
        }
        Commands::Delete { query } => {
            println!("Delete command");
        }
    }

    Ok(())
}
