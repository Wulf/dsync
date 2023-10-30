#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run -- -i schema.rs -b diesel::pg::Pg -o models -g id -g created_at -g updated_at -c "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>" --single-model-file
