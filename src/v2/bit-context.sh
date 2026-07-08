#!/bin/bash

set -ev

cargo run --features "generate_bit_context" --bin ans-bit-context -- --lookup > src/v2/bit_context.rs.new
mv src/v2/bit_context.rs.new src/v2/bit_context.rs
cargo fmt