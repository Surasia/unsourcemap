use crate::parser::{
    file::SourceMapFile,
    mapping::{process_mappings, SourceMapping},
};
use anyhow::{bail, Result};
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Version info is incorrect! Must be 3. Found: {0}")]
pub struct SourceMapVersionError(i8);

pub struct SourceMap {
    pub map_data: SourceMapFile,
    pub mappings: Vec<SourceMapping>,
}

fn create_source_map(parsed: SourceMapFile) -> Result<SourceMap> {
    if parsed.version != 3 {
        bail!(SourceMapVersionError(parsed.version));
    }

    let mappings = process_mappings(&parsed.mappings)?;

    Ok(SourceMap {
        map_data: parsed,
        mappings,
    })
}

pub fn parse_source_map(file_path: &str) -> Result<SourceMap> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let parsed: SourceMapFile = serde_json::from_reader(reader)?;

    create_source_map(parsed)
}

pub fn parse_source_map_from_string(source_map: &str) -> Result<SourceMap> {
    let parsed: SourceMapFile = serde_json::from_str(source_map)?;

    create_source_map(parsed)
}

pub fn save_source_content(save_path: &str, source_path: &str, content: &str) -> Result<()> {
    let full_path = format!("{}{}", save_path, source_path);
    let path = Path::new(&full_path);
    std::fs::create_dir_all(path.parent().unwrap())?;
    let mut file_handle = File::create(path)?;
    file_handle.write_all(content.as_bytes())?;
    Ok(())
}
