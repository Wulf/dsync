# Changelog

## next

- replace `structopt` with `clap`
- add subcommand to generate shell completions
- function `generate_files` now takes in `&Path`s instead of `PathBuf`s

## 0.0.16

- add option `schema-path` and `model-path` to specify custom paths for diesel schemas input and model output

## 0.0.15

- add option `--no-serde` to disable serde derives

## 0.0.14

- fixes for diesel `2.1.0`
