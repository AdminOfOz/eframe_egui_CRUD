#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.

set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
set -x

# Checks all tests, lints etc.
# Basically does what the CI does.

cargo install cargo-cranky # Uses lints defined in Cranky.toml. See https://github.com/ericseppanen/cargo-cranky

# web_sys_unstable_apis is required to enable the web_sys clipboard API which eframe web uses,
# as well as by the wasm32-backend of the wgpu crate.

cargo check --all-targets
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo test --doc # slow - checks all doc-tests

cargo doc --lib --no-deps --all-features
cargo doc --document-private-items --no-deps --all-features



# ------------------------------------------------------------
#

# For finding bloat:
# cargo bloat --release --bin egui_demo_app -n 200 | rg egui
# Also try https://github.com/google/bloaty

# what compiles slowly?
# cargo clean && time cargo build -p eframe --timings
# https://fasterthanli.me/articles/why-is-my-rust-build-so-slow

# what compiles slowly?
# cargo llvm-lines --lib -p egui | head -20

echo "All checks passed."