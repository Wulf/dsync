#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo run --features async -- -b diesel::pg::Pg -i schema.rs -o models -g id -g created_at -g updated_at -c "diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>" --async