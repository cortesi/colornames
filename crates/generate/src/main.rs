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
    List {
        /// Output as HTML table
        #[arg(long)]
        html: bool,
    },
    /// Generate color files
    Generate,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { html } => {
            if html {
                const COLUMNS: usize = 3;
                println!("<table style='border-collapse: collapse;'>");
                let color_data: Vec<_> = colors::COLOR_DATA.iter().collect();

                for chunk in color_data.chunks(COLUMNS) {
                    println!("<tr>");
                    for (name, hex) in chunk {
                        println!("<td style='padding: 5px;'><div style='width: 50px; height: 20px; background-color: {}'></div></td>", hex);
                        println!("<td style='padding: 5px;'>{}</td>", name);
                    }
                    println!("</tr>");
                }
                println!("</table>");
            } else {
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
        }
        Commands::Generate => {
            let enum_tokens = {
                let mut color_data: Vec<_> = colors::COLOR_DATA.iter().collect();
                color_data.sort_by_key(|(name, _)| *name);

                let color_idents: Vec<_> = color_data
                    .iter()
                    .map(|(name, _hex)| {
                        let ident = name.replace(" ", "");
                        quote::format_ident!("{}", ident)
                    })
                    .collect();

                let color_hexes: Vec<_> = color_data.iter().map(|(_, hex)| hex).collect();

                quote::quote! {
                    /// A list of named colors
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                    pub enum Color {
                        #(#color_idents),*
                    }

                    impl Color {
                        /// Convert a color name to a `Color` variant
                        pub fn from_name(name: &str) -> Option<Self> {
                            let normalized = name.replace(" ", "").to_lowercase();
                            match normalized.as_str() {
                                #(s if s == stringify!(#color_idents).to_lowercase() => Some(Color::#color_idents),)*
                                _ => None
                            }
                        }

                        /// Get the RGB values of a color
                        pub fn rgb(&self) -> Option<(u8, u8, u8)> {
                            let hex = match self {
                                #(Self::#color_idents => #color_hexes,)*
                            };
                            Some((
                                u8::from_str_radix(&hex[1..3], 16).ok()?,
                                u8::from_str_radix(&hex[3..5], 16).ok()?,
                                u8::from_str_radix(&hex[5..7], 16).ok()?
                            ))
                        }

                        /// Get the hex value of a color
                        pub fn rgb_hex(&self) -> Option<String> {
                            let (r, g, b) = self.rgb()?;
                            Some(format!("#{:02X}{:02X}{:02X}", r, g, b))
                        }
                    }
                }
            };
            println!("{}", enum_tokens);
        }
    }
}
