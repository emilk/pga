#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd "$( dirname "${BASH_SOURCE[0]}" )"

echo "[workspace]
members = [
	\"generator\",
]" > Cargo.toml

rm -rf generated/src/pga2d generated/src/pga3d

cargo run -q -- --grammar pga2d --out_dir generated/src/pga2d
cargo run -q -- --grammar pga3d --out_dir generated/src/pga3d

echo "Testing generated code:"

echo "[workspace]
members = [
	\"generated\",
	\"generator\",
]" > Cargo.toml

cargo fmt

cargo check -q --all-features
cargo test -q --all-features
cargo clippy -q
cargo clean -q --doc && cargo doc --no-deps

echo "All OK"
