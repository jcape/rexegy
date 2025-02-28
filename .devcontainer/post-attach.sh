#!/bin/bash

mkdir -p /workspaces/rxegy/.cache/cargo
mkdir -p /workspaces/rxegy/.cache/pre-commit
export CARGO_HOME=/workspaces/rxegy/.cache/cargo

cargo install -q cargo-binstall
cargo install -q cargo-semver-checks
cargo binstall -y release-plz

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null