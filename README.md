# Unofficial Rust Libraries for Exegy XCAPI

[![License][license-image][license-link]]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->[![GitHub Workflow Status][gha-image]][gha-link]<!--
-->[![Contributor Covenant][conduct-image]][conduct-link]

This is a simple project to support accessing the Exegy XCAPI API.

## Getting Started

In order to build this project, you'll need to have access to the Exegy XCAPI RPMs for RHEL 9---which means you'll need access to the Exegy support site. These should be downloaded and installed locally before starting the devcontainer so the headers and libraries are mounted into the image.

The headers are required to rebuild the generated code in `rxegy-sys`, but the libraries are required to successfully build/test. As a result, CI in this repo is fairly minimal and any unit tests, are intended to be run locally using pre-commit.

[license-image]: https://img.shields.io/crates/l/rxegy?style=flat-square
[license-link]: LICENSE
[deps-image]: https://deps.rs/repo/github/rxegy/status.svg?style=flat-square
[deps-link]: https://deps.rs/repo/github/rxegy
[gha-image]: https://img.shields.io/github/actions/workflow/status/jcape/rxegy/ci.yml?branch=main&style=flat-square
[gha-link]: https://github.com/jcape/rxegy/actions/workflows/ci.yml?query=branch%3Amain
[conduct-image]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=flat-square
[conduct-link]: CODE_OF_CONDUCT.md
