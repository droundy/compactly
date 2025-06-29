#!/bin/bash

set -ev

cargo run --features "generate_bit_context" --bin generate-bit-context -- --lookup > src/v1/bit_context.rs.new
mv src/v1/bit_context.rs.new src/v1/bit_context.rs
cargo fmt