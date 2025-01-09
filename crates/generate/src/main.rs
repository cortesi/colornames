use clap::{Parser, Subcommand};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod colors;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available colors
    List,
    /// Generate color files
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            for (name, hex) in colors::COLOR_DATA {
                let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
                let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
                let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

                stdout
                    .set_color(ColorSpec::new().set_bg(Some(Color::Rgb(r, g, b))))
                    .unwrap();
                write!(&mut stdout, "       ").unwrap();
                stdout.reset().unwrap();
                println!(" {}", name);
            }
        }
        Commands::Generate => println!("Generate command"),
    }
}
