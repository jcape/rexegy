#!/bin/bash

mkdir -p /workspaces/rxegy/.cache/cargo
if [ -d "/workspaces/rxegy/.cache/cargo/bin" ]; then
    rm -rf /workspaces/rxegy/.cache/cargo/bin
fi
ln -sf /usr/local/cargo/bin /workspaces/rxegy/.cache/cargo/
export CARGO_HOME=/workspaces/rxegy/.cache/cargo

cargo install -q cargo-binstall
cargo install -q cargo-semver-checks
cargo binstall -y release-plz

mkdir -p /workspaces/rxegy/.cache/pre-commit
export PATH=$PATH:/workspaces/rxegy/.cache/cargo/bin

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null