use clap::Parser;
use prompts_core::load_prompts;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
}

#[cfg(feature = "tui")]
mod tui;

fn main() -> anyhow::Result<()> {
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
            tui::run(file)?;
        }
    }

    Ok(())
}
