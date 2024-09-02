# unsourcemap

Rewrite of [unwebpack-sourcemap](https://github.com/rarecoil/unwebpack-sourcemap) in Rust. Recovers source files from sourcemaps generated from webpack, writing them into the specified directory.

## Usage
```
Usage: unsourcemap.exe [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>    Path to source map file (optional)
  -S, --save-path <SAVE_PATH>    Path to the location to save to (optional)
  -s, --source-map <SOURCE_MAP>  Source map content as a string (optional)
  -u, --url <URL>                URL of source map (optional)
  -h, --help                     Print help
  -V, --version                  Print version
```
