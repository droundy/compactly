#!/bin/bash

set -ev

cargo run --features "generate_bit_context" --bin ans-bit-context -- --lookup > src/ans/bit_context.rs.new
mv src/ans/bit_context.rs.new src/ans/bit_context.rs
cargo fmt