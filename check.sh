#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd "$( dirname "${BASH_SOURCE[0]}" )"

echo "[workspace]
members = [
	\"generator\",
]" > Cargo.toml

cargo check -q --all-features
cargo clippy -q
cargo run -q -- --out_dir generated/src/pga2d

echo "Testing generated code:"

echo "[workspace]
members = [
	\"generated\",
	\"generator\",
]" > Cargo.toml

cargo check -q --all-features
cargo test -q --all-features
cargo clippy -q
cargo clean -q --doc && cargo doc --no-deps

echo "All OK"
