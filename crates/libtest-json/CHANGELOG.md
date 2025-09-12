# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

- Make schema `snake_case`
- Timing is optional and is time since test start
- Made fields with well known defaults to be optional
- Made `DiscoverCase` order unspecified
- Renamed `DiscoverCase::run` to `selected`
- Rename `Suite` to `Run`
- Rename `RunStatus` to `MessageKind`
- Rename `MessageKind::Fail` to `MessageKind::Error`
- Moved content from `CaseComplete` to `CaseMessage`

## [0.0.1] - 2025-07-18

<!-- next-url -->
[Unreleased]: https://github.com/epage/pytest-rs/compare/libtest-json-v0.0.1...HEAD
[0.0.1]: https://github.com/rust-cli/argfile/compare/c96ef27899b410f9f154183989d4ccf60af27da6...libtest-json-v0.0.1
