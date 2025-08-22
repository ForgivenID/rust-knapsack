use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use std::io::{self, Write};

mod network;
mod storage;
mod video;

use video::VideoProcessor;

#[derive(Parser)]
#[command(name = "kpsk")]
#[command(about = "KnapSack - Distributed video sharing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Prepare media for sharing
    Prep {
        #[arg(required = true)]
        media_path: PathBuf,
    },
    /// Advertise media metadata
    Add {
        #[arg(short, long)]
        prep: bool,
        #[arg(required = true)]
        file_path: PathBuf,
    },
    /// Find videos matching query
    Find {
        #[arg(short)]
        subscription: bool,
        #[arg(required = true)]
        count: usize,
        search_query: Vec<String>,
    },
    /// View a video
    View {
        #[arg(short, long)]
        preload: bool,
        #[arg(long)]
        autoplay: bool,
        query: String,
    },
    /// Exit the application
    Exit,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Initialize network manager
    let mut network = network::NetworkManager::new()
        .map_err(|e| anyhow::anyhow!("Network initialization failed: {}", e))?;

    // Start network in background
    tokio::spawn(async move {
        network.run().await;
    });

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        // Execute single command and exit
        execute_command(command).await?;
    } else {
        // Enter interactive mode
        interactive_mode().await?;
    }

    Ok(())
}

async fn interactive_mode() -> Result<()> {
    println!("KnapSack - Distributed video sharing");
    println!("Type 'help' for available commands, 'exit' to quit.");

    loop {
        print!("kpsk> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Parse the input as if it were command line arguments
        let args = shlex::split(input).unwrap_or_else(|| {
            input.split_whitespace().map(|s| s.to_string()).collect()
        });

        // Add the program name as the first argument
        let args = std::iter::once("kpsk".to_string()).chain(args);

        match Cli::try_parse_from(args) {
            Ok(cli) => {
                if let Some(command) = cli.command {
                    if matches!(command, Commands::Exit) {
                        println!("Goodbye!");
                        break;
                    }

                    if let Err(e) = execute_command(command).await {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                if input == "help" {
                    print_help();
                } else {
                    eprintln!("Error: {}", e);
                    println!("Type 'help' for available commands.");
                }
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  prep <MEDIA_PATH>      - Prepare media for sharing");
    println!("  add [--prep] <FILE_PATH> - Advertise media metadata");
    println!("  find [-s] <N> <QUERY>  - Find videos matching query");
    println!("  view [--preload] [--autoplay] <QUERY> - View a video");
    println!("  exit                   - Exit the application");
    println!();
    println!("Options:");
    println!("  --prep, -p             - Prepare media before advertising");
    println!("  --subscription, -s     - Prefer subscriptions (WIP)");
    println!("  --preload, -p          - Don't open player until all pieces are collected");
    println!("  --autoplay, --ap       - Start playing as soon as player opens");
}

async fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Prep { media_path } => {
            prep_video(&media_path).await?;
        }
        Commands::Add { prep, file_path } => {
            add_video(prep, &file_path).await?;
        }
        Commands::Find { subscription, count, search_query } => {
            find_videos(subscription, count, search_query).await?;
        }
        Commands::View { preload, autoplay, query } => {
            view_video(preload, autoplay, &query).await?;
        }
        Commands::Exit => {
            // Handled in interactive_mode
        }
    }

    Ok(())
}

async fn prep_video(media_path: &PathBuf) -> Result<()> {
    println!("Preparing video at: {:?}", media_path);
    let metadata = VideoProcessor::prepare_video(media_path)?;
    println!("Created metadata for video: {}", metadata.hash);

    // Save metadata to file
    let metadata_json = serde_json::to_string(&metadata)?;
    let metadata_path = media_path.with_extension("json");
    std::fs::write(metadata_path, metadata_json)?;

    println!("Video prepared successfully!");
    Ok(())
}

async fn add_video(prep: bool, file_path: &PathBuf) -> Result<()> {
    println!("Adding video at: {:?} (prep: {})", file_path, prep);

    if prep {
        prep_video(file_path).await?;
    }

    // Load metadata
    let metadata_path = file_path.with_extension("json");
    let metadata_json = std::fs::read_to_string(metadata_path)?;
    let metadata: video::VideoMetadata = serde_json::from_str(&metadata_json)?;

    println!("Advertising video: {}", metadata.title);
    // TODO: Implement network advertising

    Ok(())
}

async fn find_videos(subscription: bool, count: usize, search_query: Vec<String>) -> Result<()> {
    println!("Finding {} videos (subscription: {})", count, subscription);
    println!("Search query: {:?}", search_query);
    // TODO: Implement video discovery
    Ok(())
}

async fn view_video(preload: bool, autoplay: bool, query: &str) -> Result<()> {
    println!("Viewing video: {} (preload: {}, autoplay: {})", query, preload, autoplay);
    // TODO: Implement video viewing
    Ok(())
}