use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
/// Structure of the source map file in JSON.
/// Read more here:
/// * <https://www.bugsnag.com/blog/source-maps>
/// * <https://github.com/tc39/source-map/blob/main/source-map-rev3.md>
/// * <https://tc39.es/source-map>
pub struct SourceMapFile {
    pub version: i8,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    pub file: Option<String>,
    #[serde(rename = "sourcesContent")]
    pub sources_content: Option<Vec<String>>,
    #[serde(rename = "sourceRoot")]
    pub source_root: Option<String>,
    #[serde(rename = "ignoreList")]
    pub ignore_list: Option<Vec<i32>>,
}
