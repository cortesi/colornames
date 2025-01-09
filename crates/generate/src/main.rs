use clap::{Parser, Subcommand};
use image_builder::{Image, Rect};
use std::{io::Write, path::Path};
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
    /// Generate PNG swatches for all colors
    Swatches {
        /// Directory to write swatch files
        directory: String,
    },
    /// Generate HTML with remote swatch images
    Readme,
}

const COLUMNS: usize = 3;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Readme => {
            println!("<table style='border-collapse: collapse;'>");
            let color_data: Vec<_> = colors::COLOR_DATA.iter().collect();

            for chunk in color_data.chunks(COLUMNS) {
                println!("<tr>");
                for (name, _hex) in chunk {
                    let swatch_name = name.replace(" ", "_").to_lowercase();
                    let swatch_url = format!(
                        "https://raw.githubusercontent.com/cortesi/colornames/refs/heads/main/swatches/{}.png",
                        swatch_name
                    );
                    println!("<td style='padding: 5px;'><img src=\"{}\" width=\"50\" height=\"20\"></td>", swatch_url);
                    println!("<td style='padding: 5px;'>{}</td>", name);
                }
                println!("</tr>");
            }
            println!("</table>");
        }
        Commands::List { html } => {
            if html {
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
        Commands::Swatches { directory } => {
            let dir_path = Path::new(&directory);
            if !dir_path.is_dir() {
                eprintln!("Error: {} is not a directory", directory);
                std::process::exit(1);
            }

            for (name, hex) in colors::COLOR_DATA {
                let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
                let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
                let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

                let mut image = Image::new(20, 20, [255, 255, 255, 255]);
                image.add_rect(
                    Rect::new()
                        .size(20, 20)
                        .position(0, 0)
                        .color([r, g, b, 255]),
                );

                let filename = format!("{}.png", name.replace(" ", "_").to_lowercase());
                let path = dir_path.join(filename);
                image.save(path.to_str().unwrap());
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
                        pub fn rgb(&self) -> (u8, u8, u8) {
                            let hex = self.rgb_hex();
                            (
                                // Unwraps can't fail because the hex values are hardcoded
                                u8::from_str_radix(&hex[1..3], 16).unwrap(),
                                u8::from_str_radix(&hex[3..5], 16).unwrap(),
                                u8::from_str_radix(&hex[5..7], 16).unwrap()
                            )
                        }

                        /// Get the hex value of a color
                        pub fn rgb_hex(&self) -> String {
                            match self {
                                #(Self::#color_idents => #color_hexes,)*
                            }.to_string()
                        }

                        /// Get the name of the color as a string
                        pub fn name(&self) -> &'static str {
                            match self {
                                #(Self::#color_idents => stringify!(#color_idents),)*
                            }
                        }
                    }
                }
            };
            println!("{}", enum_tokens);
        }
    }
}
