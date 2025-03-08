#!/bin/bash

npm install -g @withgraphite/graphite-cli@stable

# Install miri
rustup toolchain install --profile=miinimal nightly
rustup component add --toolchain nightly miri rust-src
