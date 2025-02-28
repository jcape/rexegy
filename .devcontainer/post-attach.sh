#!/bin/bash

mkdir -p /workspaces/rxegy/.cache/cargo
mkdir -p /workspaces/rxegy/.cache/pre-commit
export CARGO_HOME=/workspaces/rxegy/.cache/cargo

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null