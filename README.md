# Aggregate json

This is a simple json aggregator in rust utilizing serde_json. This was one of the things used for maintaining a custom crypto trading system and debugging.

The merging strategy used here is adding arrays together and adding together objects (including nested ones) replacing existing keys with new ones when we reach a finite value.

# Usage

Usage: cargo run --release -- <input_file1> <input_file2> ... <output_file>
