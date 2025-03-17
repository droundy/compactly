#!/bin/bash

set -ev

cargo run --example bit-context > src/v1/bit_context.rs.new
mv src/v1/bit_context.rs.new src/v1/bit_context.rs
cargo fmt