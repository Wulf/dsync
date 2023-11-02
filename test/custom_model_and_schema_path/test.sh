#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run --manifest-path ../../Cargo.toml -- \
-i data/schema.rs -o data/models -g id -g created_at -g updated_at -c "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>" --schema-path "crate::data::schema::" --model-path "crate::data::models::"
