use clap::Parser;
use prompts_core::{load_prompts, MockTextGenerator, TextGenerator, LLMTextGenerator, GeneratorType};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Choose the text generation backend
    #[arg(long, value_enum, default_value_t = GeneratorType::Mock)]
    generator: GeneratorType,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Lists all the prompts
    List {
        /// The path to the prompts file
        #[arg(short, long)]
        file: String,
    },
    /// Shows a specific prompt
    Show {
        /// The name of the prompt to show
        name: String,

        /// The path to the prompts file
        #[arg(short, long)]
        file: String,
    },
    #[cfg(feature = "tui")]
    /// Starts the interactive TUI
    Tui {
        /// The path to the prompts file
        #[arg(short, long)]
        file: String,
    },
    /// Generates text based on a prompt
    Generate {
        /// The name of the prompt to use for generation
        name: String,

        /// The path to the prompts file
        #[arg(short, long)]
        file: String,
    },
}

#[cfg(feature = "tui")]
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { file } => {
            let prompts = load_prompts(file)?;
            for prompt in prompts {
                println!("{}: {}", prompt.name, prompt.text);
            }
        }
        Commands::Show { name, file } => {
            let prompts = load_prompts(file)?;
            if let Some(prompt) = prompts.iter().find(|p| p.name == *name) {
                println!("{}", prompt.text);
            } else {
                anyhow::bail!("Prompt '{}' not found", name);
            }
        }
        #[cfg(feature = "tui")]
        Commands::Tui { file } => {
            tui::run(file, cli.generator).await?;
        }
        Commands::Generate { name, file } => {
            let prompts = load_prompts(file)?;
            if let Some(prompt) = prompts.iter().find(|p| p.name == *name) {
                let generated_text = match cli.generator {
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
    }

    Ok(())
}