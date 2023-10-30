#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run -- -i schema.rs -b diesel::pg::Pg -o models -g id -c "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>"
