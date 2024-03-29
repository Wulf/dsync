#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run -F advanced-queries --manifest-path ../../Cargo.toml -- \
--diesel-backend diesel::pg::Pg -i schema.rs -o models -g id -g created_at -g updated_at -c "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>"
