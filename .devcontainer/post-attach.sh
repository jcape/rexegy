#!/bin/bash

npm install -g @withgraphite/graphite-cli@stable
cargo install -q cargo-binstall
cargo install -q cargo-semver-checks
cargo binstall -y release-plz

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null