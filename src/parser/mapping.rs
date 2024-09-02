use super::vlq::decode_vlq;
use anyhow::Result;

#[derive(Debug, Default)]
/// Structure of the source mapping definition in Base64 VLQ.
/// Read more here:
/// * <https://www.bugsnag.com/blog/source-maps>
/// * <https://github.com/tc39/source-map/blob/main/source-map-rev3.md>
/// * <https://tc39.es/source-map>
pub struct SourceMapping {
    pub generated_line: i32,
    pub generated_column: i32,
    pub original_line: Option<i32>,
    pub original_column: Option<i32>,
    pub source_index: Option<i32>,
    pub name_index: Option<i32>,
}

/// Maps the Base64 VLQ values into source mapping types.
///
/// Processes the mappings string, which contains Base64 VLQ encoded
/// information about the source map.
///
/// # Arguments
///
/// * `mappings` - A string slice that holds the Base64 VLQ encoded mappings
///
/// # Returns
///
/// * `Result<Vec<SourceMapping>>` - A vector of SourceMapping structs if successful
pub fn process_mappings(mappings: &str) -> Result<Vec<SourceMapping>> {
    let mut result = Vec::new();
    let mut generated_column: i32;
    let mut source_index = 0;
    let mut original_line = 0;
    let mut original_column = 0;
    let mut name_index = 0;

    for (generated_line, line) in mappings.split(';').enumerate() {
        generated_column = 0; // Reset for each new line

        for segment in line.split(',') {
            let values = decode_vlq(segment)?;
            if values.is_empty() {
                continue;
            }

            generated_column += values[0];

            let mut mapping = SourceMapping {
                generated_line: generated_line as i32,
                generated_column,
                ..Default::default() // fill out the rest of the entries with default values
            };

            if values.len() > 1 {
                // Fields 2,3,4 exist
                source_index += values[1];
                original_line += values[2];
                original_column += values[3];
                mapping.source_index = Some(source_index);
                mapping.original_line = Some(original_line);
                mapping.original_column = Some(original_column);

                if values.len() > 4 {
                    // Field 5 exists
                    name_index += values[4];
                    mapping.name_index = Some(name_index);
                }
            }

            result.push(mapping);
        }
    }
    Ok(result)
}
