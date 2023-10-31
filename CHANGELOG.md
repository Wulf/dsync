# Changelog

## next

- replace `structopt` with `clap`
- replace dependency `Inflector` with `heck`
- add dependency `thiserror`
- remove dependency `anyhow`
- add subcommand to generate shell completions
- function `generate_files` now takes in `&Path`s instead of `PathBuf`s
- remove `to_singular` name generations
- add option `--create-str` to set `Create*` structs string type
- add option `--update-str` to set `Update*` structs string type
- add option `--single-model-file` to only generate a single file instead of a directory with `mod.rs` and `generated.rs`
- add option `--readonly-prefix` and `--readonly-suffix` to treat a matching name as a readonly struct
- add option `--no-crud` to not generate any `impl` blocks
- derive generation has been refactored and now only necessary derives are added to a given struct
- reduce amount of empty-newlines
- add `dsync::Error`(and `dsync::error` module)
- replace most `panic!`, `.expect` and some `.unwrap` with `Result`s
- fix nullable unsigned integers not resulting in `Option<u*>`
- derive `Default` for all `Update*` structs
- use `@generated` file signature to exclude the files from formatting
- rename `type Connection =` to `type ConnectionType =` to lessen naming conflicts
- add many doc-comments to fields and functions
- list changes to files (unchanged, modified, deleted)
- generate doc-comments for generated structs, fields and functions
- change the diesel import to be `use diesel` (instead of the previous `use crate::diesel`)

## 0.0.17 (yanked)

- used for testing publishing CI

## 0.0.16

- add option `schema-path` and `model-path` to specify custom paths for diesel schemas input and model output

## 0.0.15

- add option `--no-serde` to disable serde derives

## 0.0.14

- fixes for diesel `2.1.0`
