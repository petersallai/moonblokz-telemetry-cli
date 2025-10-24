mod config;
mod parser;
mod client;

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use std::io::{self, Write};

use config::Config;
use parser::{parse_command, Command};
use client::Client;

#[derive(ClapParser, Debug)]
#[command(name = "moonblokz-telemetry-cli")]
#[command(about = "MoonBlokz Telemetry CLI - Send commands to probes via the telemetry hub", long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(long, default_value = "config.toml")]
    config: String,
    
    /// Single command to send and exit
    #[arg(long)]
    command: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load configuration
    let config = Config::load(&args.config)
        .context(format!("Failed to load configuration from {}", args.config))?;
    
    // Create client
    let client = Client::new(config)?;
    
    // Single command mode or interactive mode
    if let Some(command_str) = args.command {
        // Single command mode
        execute_single_command(&client, &command_str).await
    } else {
        // Interactive mode
        interactive_mode(&client).await
    }
}

async fn execute_single_command(client: &Client, command_str: &str) -> Result<()> {
    match parse_command(command_str) {
        Ok(Command::Quit) => {
            eprintln!("Quit command is only valid in interactive mode");
            std::process::exit(1);
        }
        Ok(command) => {
            match client.send_command(&command).await {
                Ok(result) => {
                    println!("{}", result);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn interactive_mode(client: &Client) -> Result<()> {
    println!("MoonBlokz Telemetry CLI - Interactive Mode");
    println!("Type 'quit', 'exit', or 'bye' to exit");
    println!();
    
    loop {
        // Print prompt
        print!("> ");
        io::stdout().flush()?;
        
        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("\nError reading input");
            continue;
        }
        
        let input = input.trim();
        
        // Skip empty lines
        if input.is_empty() {
            continue;
        }
        
        // Parse command
        match parse_command(input) {
            Ok(Command::Quit) => {
                println!("Goodbye!");
                break;
            }
            Ok(command) => {
                // Send command
                match client.send_command(&command).await {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        // Check if it's an authentication error - if so, exit
                        if e.to_string().contains("401 Unauthorized") {
                            eprintln!("Authentication failed. Please check your API key in the config file.");
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
    }
    
    Ok(())
}
