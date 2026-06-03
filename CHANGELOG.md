# Changelog: aki-gsub
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Test `$n` capture group syntax compatibility (e.g., `-f "$1"`)
- Perform comprehensive code review (saved to `docs/reviews/2026-06-01_code_review.3.md`)

### Changed
- Reorganize and rename past code review documents in `docs/reviews/`
- Document sequential replacement behavior in README and module documentation
- Optimize regex substitution logic by removing redundant `is_match` calls
- Refactor coloring logic into a reusable `colorize` utility function
- Optimize memory allocation in `do_match_proc` by reusing a line buffer

## [0.2.1] - 2026-05-18

### Added
- Standardize capture group references to `${n}` syntax (e.g., `${1}`) in documentation and tests
- Perform comprehensive code review (saved to `docs/review.2.md`)

### Changed
- Refactor `EnvConf` to reduce code duplication and improve maintainability
- Refactor regex substitution logic to a sequential application strategy (similar to `sed`)
- Update design documentation (`specs/2.design.md`) to reflect the sequential substitution strategy
- Minimum supported rustc 1.68.0 (2c8cc3432 2023-03-06)
- Update crates: flood-tide (0.2.14), flood-tide-gen (0.2.2)
- Update crates: runnel (0.4.2), regex (1.12)
- Refactor format string parsing to use `regex::Captures::expand`
- Update tests to use `${n}` syntax for `Captures::expand` compatibility

### Fixed
- Help message formatting to match test expectations
- Clippy: `unnecessary_sort_by`
- `x_rvi_msg!()`
- Infinite loop bug in manual format parsing logic

### Removed
- `memx-cdy`

## [0.2.0] - 2025-09-15

### Added
- `specs` directory
- More tests, including invalid UTF-8 input test
- `execute_with_env()`

### Changed
- `IntoIterator` compatibility for args in `execute()`
- Update crates: runnel (0.4.0), rust-version-info-file (0.2.0), regex (1.11)
- Refactor `lib.rs`

### Fixed
- Minimum support version in documentation
- Bug: replacement is "$$"
- Bug: named capture groups
- Bug: empty match

### Removed
- `execute_env()`
- `base_dir=` of `-X` options

## [0.1.38] - 2024-06-20

### Added
- GitHub Actions workflows: `.github/workflows/test-{ubuntu,macos,windows}.yml`
- Test status badges in `README.tpl`
- Miri support in tests
- Tarpaulin support in `Makefile`
- `-X` option tests and other various tests

### Changed
- Rename `config` to `config.toml`
- Remove `cfg(has_not_matches)`
- Refactor `Makefile`
- Update dependencies: flood-tide (0.2.11), flood-tide-gen (0.1.22), memx-cdy (0.1.13), runnel (0.3.19), exec-target (0.2.8), indoc (2.0.0), rust-version-info-file (0.1.10)

### Fixed
- Bug: `test_x_option_rvi()`
- License files: `LICENSE-APACHE`, `LICENSE-MIT`
- Clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `borrow_deref_ref`, `unused_imports`, `derivable_impls`
- Update rust-version from "1.56.0" to "1.65.0"

### Removed
- `OptColorWhenParseError::description()` (deprecated)
- `OptUcXParamParseError::description()` (deprecated)
- `COPYING`

## [0.1.37] - 2023-01-11

### Fixed
- HTTP links in `CHANGELOG.md`

## [0.1.36] - 2023-01-11 [YANKED]

### Added
- Badges in `README.tpl`
- `rust-version = "1.56.0"` to `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependencies: anyhow (1.0.68), flood-tide (0.2.8), flood-tide-gen (0.1.19), memx-cdy (0.1.10), runnel (0.3.15), regex (1.7.1)

### Fixed
- Clippy: you are deriving `PartialEq` and can implement `Eq`
- Clippy: `uninlined_format_args`, `manual_is_ascii_check`

## [0.1.35] - 2022-06-18

### Changed
- Change to Edition 2021
- Update dependencies: flood-tide (0.2.5), memx (0.1.21), memx-cdy (0.1.8), runnel (0.3.11), exec-target (0.2.6), flood-tide-gen (0.1.16), rust-version-info-file (0.1.6), semver (1.0.10)

## [0.1.34] - 2022-05-22

### Changed
- Update dependencies: runnel (0.3.10), anyhow (1.0.57), libc (0.2.126), regex (1.5.6), exec-target (0.2.5), rust-version-info-file (0.1.5)

## [0.1.33] - 2021-11-15

### Added
- More documentation

### Changed
- Minimum supported rustc 1.47.0 (18bf6b4f0 2020-10-07)
- Update dependencies: flood-tide (0.2.4), memx (0.1.18), memx-cdy (0.1.7), runnel (0.3.9), anyhow (1.0.45), libc (0.2.107), exec-target (0.2.4), flood-tide-gen (0.1.15), rust-version-info-file (0.1.3)

## [0.1.32] - 2021-09-11

### Added
- Dependency: indoc (1.0.3)

### Changed
- Pass Cargo Clippy
- Update dependencies: anyhow (1.0.43), flood-tide-gen (0.1.14), flood-tide (0.2.3), memx-cdy (0.1.6), runnel (0.3.8), exec-target (0.2.3)
- Rewrite `TARGET_EXE_PATH` with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`

## [0.1.31] - 2021-06-24

### Added
- `memx_cdy::memx_init()` for fast memory operations

### Changed
- Rewrite `TARGET_EXE_PATH` with `env!("CARGO_BIN_EXE_aki-gsub")`

### Fixed
- Bug: `#[cfg(feature = "debian_build")]`

## [0.1.30] - 2021-06-03

### Added
- Support for `features = ["debian_build"]`

### Changed
- Update dependencies: flood-tide (0.2.2), regex (1.5.4)

### Fixed
- Bug: command option `-X rust-version-info`

## [0.1.29] - 2021-04-23

### Added
- Command option `-X`

### Changed
- Update dependencies: flood-tide-gen (0.1.12), flood-tide (0.2.1)
- Bug fix update: regex (1.4.6)

## [0.1.28] - 2021-04-19

### Changed
- Update dependency: flood-tide-gen (0.1.10)

## [0.1.27] - 2021-04-07

### Changed
- Update dependency: flood-tide (0.2)
- Update dependencies: anyhow (1.0.40), flood-tide-gen (0.1.8), runnel (0.3.6)

## [0.1.26] - 2021-03-22

### Added
- `--color <when>`
- Content to `--help`

### Changed
- Update dependency: regex 1.4.5 (fixes stack overflows)

## [0.1.25] - 2021-03-14

### Changed
- Update dependency: regex (fix memory leak)

## [0.1.24] - 2021-03-08

### Changed
- Update dependency: runnel
- Update dependency: rustc_version (0.3)

## [0.1.23] - 2021-03-08

### Changed
- Update dependency: runnel
- Rename file: `xtask/src/cmd.txt` to `xtask/src/aki-gsub-cmd.txt`
- Cleanup `src/main.rs` and `build.rs`

## [0.1.22] - 2021-03-02

### Added
- More documentation

### Changed
- Change option: `-e, --expression` to `-e, --exp`
- Update dependency: flood-tide-gen

### Fixed
- Bug: add `flush()` on finish

## [0.1.21] - 2021-02-22

### Added
- More documentation

### Changed
- Update dependencies: runnel, flood-tide-gen

### Fixed
- Bug: added `flush()` on finish

## [0.1.20] - 2021-02-14

### Added
- Documentation

### Changed
- Update dependency: runnel
- Rename section "AAA-admin" to "AAA-text" of `package.metadata.deb`

## [0.1.19] - 2021-02-07

### Changed
- Update dependency: flood-tide-gen

## [0.1.18] - 2021-02-05

### Changed
- Update dependency: runnel

## [0.1.17] - 2021-02-05

### Fixed
- Bug in `README.md`

## [0.1.16] - 2021-02-05

### Changed
- Initial GitHub release

## 0.1.15 - 2021-02-05

### Added
- Import crate `exec-target` from local for testing

## 0.1.14 - 2021-01-31

### Changed
- Change `AppError` to `anyhow::Error`
- Change configuration parser to `flood-tide` and `flood-tide-gen`
- General refactoring

## 0.1.13 - 2021-01-24

### Added
- `matches!()` macro support before rustc 1.42.0
- `cfg(has_fat_stdout)` and test support before rustc 1.44.0

## 0.1.12 - 2021-01-24

### Added
- `pipeio`

### Changed
- Rename `streamio` to `runnel`

## 0.1.11 - 2021-01-22

### Changed
- Refactor `stream` module

## 0.1.10 - 2021-01-19

### Added
- Tests with `stream` module

## 0.1.9 - 2021-01-17

### Added
- `xtask`
- `stream` module

### Changed
- Change `optpa_util_1` to `flood-tide`

## 0.1.8 - 2020-12-29

### Changed
- Update dependencies

### Removed
- `optpaerr-1`

## 0.1.7 - 2020-11-17

### Added
- `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`

### Changed
- Change `optpa_util` to `optpa_util_1`

### Fixed
- Old version: `rustc_version` (=0.2.3), v0.3.0 does not compile new semver on deb10-buster

## 0.1.6 - 2020-08-09

### Added
- Support for `cargo deb`

### Changed
- Update dependencies

## 0.1.5 - 2020-05-10

### Changed
- Change Edition 2015 to 2018
- Update dependencies

## 0.1.4 - 2020-03-30

### Added
- Support for broken pipe and corresponding tests

### Changed
- Update dependencies

## 0.1.3 - 2019-04-14

### Added
- Support for `std::alloc`

### Changed
- Update dependencies

## 0.1.2 - 2018-05-04

### Added
- Support for `cfg(has_global_allocator)`

### Changed
- Update dependencies

## 0.1.1 - 2018-03-22

### Added
- Support for broken pipe
- Miscellaneous additions

### Changed
- Update dependencies

## 0.1.0 - 2017-12-12

### Added
- First commit

[Unreleased]: https://github.com/aki-akaguma/aki-gsub/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-gsub/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.38..v0.2.0
[0.1.38]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.37..v0.1.38
[0.1.37]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.36..v0.1.37
[0.1.36]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.35..v0.1.36
[0.1.35]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.34..v0.1.35
[0.1.34]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.33..v0.1.34
[0.1.33]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.32..v0.1.33
[0.1.32]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.31..v0.1.32
[0.1.31]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.30..v0.1.31
[0.1.30]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.29..v0.1.30
[0.1.29]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.28..v0.1.29
[0.1.28]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.27..v0.1.28
[0.1.27]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.26..v0.1.27
[0.1.26]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.25..v0.1.26
[0.1.25]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-gsub/releases/tag/v0.1.16
