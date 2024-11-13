use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "html_parser")]
#[command(version = "0.1.0")]
#[command(about = "A simple HTML parser implemented in Rust using Pest.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { file: String },
    Help,
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let content = fs::read_to_string(file)
                .with_context(|| format!("Failed to read file: {}", file))?;
            let parse_result = html_parser::parse_html(&content)
                .with_context(|| "Failed to parse HTML content")?;
            println!("{:#?}", parse_result);
        }
        Commands::Help => {
            hrlp();
        }
        Commands::Credits => {
            println!("HTML Parser розроблено Тарасенко Михайлом.");
        }
    }

    Ok(())
}

fn hrlp() {
    println!("HTML Parser CLI");
    println!();
    println!("Використання:");
    println!("  html_parser <COMMAND> [ARGS]");
    println!();
    println!("Доступні команди:");
    println!("  parse <FILE_PATH>    Парсити HTML файл");
    println!("  help                 Показати цю допомогу");
    println!("  credits              Показати інформацію про авторів");
    println!();
    println!("Приклади:");
    println!("  html_parser parse example.html");
    println!("  html_parser help");
    println!("  html_parser credits");
}
