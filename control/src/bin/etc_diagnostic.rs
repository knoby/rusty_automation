use std::io::{self, Write};

use anyhow::Result;
use log::debug;
use simplelog::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Init Logger
    let log_file = std::fs::File::create("etc_diag_log.txt")?;
    simplelog::WriteLogger::init(log::LevelFilter::Debug, Config::default(), log_file)?;

    run_repl()
}

fn run_repl() -> Result<()> {
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
                Commands::ListPorts => print_interfaces()?,
            },
            Err(_) => println!("Unknown Command. use help for list of commands"),
        }
    }

    Ok(())
}

fn print_help() {
    println!("Available commands:");
    println!("  help                 – show this help");
    println!("  exit / quit          – end program");
    println!("  list_ports           - Show list of available ethernet ports");
    println!("help <command> displays information about the command and its use")
}

fn print_interfaces() -> Result<()> {
    let ifaces = if_addrs::get_if_addrs()?;

    for iface in ifaces {
        println!("--- Interface -----------------------------------");
        println!("Name     : {}", iface.name);
        match iface.addr {
            if_addrs::IfAddr::V4(v4) => {
                println!("IPv4          : {}", v4.ip);
                if let Some(broadcast) = v4.broadcast {
                    println!("  Broadcast   : {}", broadcast);
                }
            }
            if_addrs::IfAddr::V6(v6) => {
                println!("IPv6          : {}", v6.ip);
            }
        }
        println!();
    }
    Ok(())
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
