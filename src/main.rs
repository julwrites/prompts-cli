use clap::Parser;
use prompts_core::{load_prompts, Prompt};

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { file } => {
            let prompts = load_prompts(file)?;
            for prompt in prompts {
                println!("{}: {}", prompt.name, prompt.text);
            }
        }
    }

    Ok(())
}

