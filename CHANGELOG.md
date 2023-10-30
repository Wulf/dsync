# Changelog

## next

- replace `structopt` with `clap`
- add subcommand to generate shell completions
- function `generate_files` now takes in `&Path`s instead of `PathBuf`s
- remove `to_singular` name generations
- replace dependency `Inflector` with `heck`
- add option `create-str` to set `Create*` structs string type
- add option `update-str` to set `Update*` structs string type
- add option `--single-model-file` to only generate a single file instead of a directory with `mod.rs` and `generated.rs`
- add option `--readonly-prefix` and `--readonly-suffix` to treat a matching name as a readonly struct
- derive generation has been refactored and now only necessary derives are added to a given struct
- reduce amount of empty-newlines

## 0.0.16

- add option `schema-path` and `model-path` to specify custom paths for diesel schemas input and model output

## 0.0.15

- add option `--no-serde` to disable serde derives

## 0.0.14

- fixes for diesel `2.1.0`
