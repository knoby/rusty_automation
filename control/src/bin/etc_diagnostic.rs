use std::io::{self, Write};

use log::debug;
use simplelog::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init Logger
    let log_file = std::fs::File::create("etc_diag_log.txt")?;
    simplelog::WriteLogger::init(log::LevelFilter::Debug, Config::default(), log_file)?;

    run_repl()
}

fn run_repl() -> anyhow::Result<()> {
    debug!("Starting Repl Loop");
    println!("Welcom to Rusty Automation EtherCat Diagnostic Tool");
    let stdin = io::stdin();
    loop {
        print!("> "); // Prompt
        io::stdout().flush()?; // Display direct 

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            // EOF (Ctrl‑D) → End
            break;
        }

        // Ignore emtpy lines
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Aufteilen wie in einer Shell (unterstützt Anführungszeichen)
        let parts = match shell_words::split(trimmed) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Parse‑Fehler: {e}");
                continue;
            }
        };

        // Das erste Wort ist das Kommando
        let cmd = parts[0].as_str().try_into();
        match cmd {
            Ok(cmd) => match cmd {
                Commands::Help => print_help(),
                Commands::Exit => break,
                Commands::ListPorts => todo!("Implement Port Scanner"),
            },
            Err(_) => println!("Unknown Command. use help for list of commands"),
        }
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  help                 – diese Hilfe");
    println!("  exit / quit          – Programm beenden");
    println!("help <command> displays information about the command and its use")
}

enum Commands {
    Exit,
    ListPorts,
    Help,
}

impl TryFrom<&str> for Commands {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "exit" | "Exit" | "quit" | "Quit" => Ok(Self::Exit),
            "help" | "Help" => Ok(Self::Help),
            "ListPorts" | "list_ports" => Ok(Self::ListPorts),
            _ => Err(anyhow::anyhow!("Unknown Command")),
        }
    }
}
