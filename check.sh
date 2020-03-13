#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

cd "$( dirname "${BASH_SOURCE[0]}" )"

cargo check --all-features
cargo clippy
cargo run -- --out_dir generated/src/pga2d
