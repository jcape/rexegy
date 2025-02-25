#!/bin/bash

npm install -g @withgraphite/graphite-cli@stable
cargo install cargo-binstall
cargo binstall -y release-plz
cargo binstall -y cargo-semver-checks

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null