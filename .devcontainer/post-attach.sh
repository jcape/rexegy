#!/bin/bash

npm install -g @withgraphite/graphite-cli@stable

pushd /workspaces/rxegy >/dev/null
pre-commit install >/dev/null
popd >/dev/null