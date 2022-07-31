# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3] - 2022-07-31

### Changed
- `turbolib` is now published on (crates.io)[crates.io]. 
- `turbolib` now uses Arc types to avoid unecessary string clones


## [0.3.2] - 2022-07-06
### Added
- `turbolib`: crate that serves as a backend for `turbo` and `py-turbo` as well as a stand-alone Rust library
- `py-turbo`: python bindings for the newly added `turbolib`
- this changelog
- new actions to release python bindings

### Changed
-  `turbo` now depends on `turbolib`
