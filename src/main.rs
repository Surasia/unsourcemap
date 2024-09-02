pub mod loader;
pub mod parser;

use anyhow::Result;
use clap::Parser;
use loader::{parse_source_map, parse_source_map_from_string, save_source_content};

#[derive(Parser, Debug)]
#[command(name = "Javascript Source Map Parser")]
#[command(version, about)]
/// Small command line utility to work with parsing JavaScript source map files.
struct Unsourcemap {
    #[arg(short, long)]
    /// Path to source map file (optional).
    file_path: Option<String>,
    #[arg(short = 'S', long)]
    /// Path to the location to save to (optional).
    save_path: Option<String>,
    #[arg(short, long)]
    /// Source map content as a string (optional).
    source_map: Option<String>,
    #[arg(short, long)]
    /// URL of source map (optional).
    url: Option<String>,
}

fn main() -> Result<()> {
    let args = Unsourcemap::parse();
    let parsed = if let Some(file_path) = args.file_path {
        parse_source_map(&file_path)?
    } else if let Some(source_map) = args.source_map {
        parse_source_map_from_string(&source_map)?
    } else if let Some(url) = args.url {
        let response = minreq::get(url).send()?;
        parse_source_map_from_string(response.as_str()?)?
    } else {
        anyhow::bail!("Either file_path or source_map must be provided");
    };

    if let Some(sources_content) = parsed.map_data.sources_content {
        for (index, source_path) in parsed.map_data.sources.iter().enumerate() {
            if let Some(content) = sources_content.get(index) {
                match &args.save_path {
                    None => {
                        println!("[FILE] {}\n\n {}\n", source_path, content);
                    }
                    Some(save_path) => {
                        save_source_content(save_path, source_path, content)?;
                    }
                }
            }
        }
    }
    Ok(())
}
