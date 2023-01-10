# Changelog: aki-gsub

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Added
* badges into `README.tpl`

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: regex(1.7.1)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: uninlined_format_args, manual_is_ascii_check


## [0.1.35] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)

## [0.1.34] (2022-05-22)
### Changed
* update depends: runnel(0.3.10)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.33] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.32] (2021-09-11)
### Added
* add depends: indoc(1.0.3)

### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.31] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-gsub")`

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.30] (2021-06-03)
### Added
* support features = \["debian_build"\]

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug: command option: -X rust-version-info

## [0.1.29] (2021-04-23)
### Added
* command option: -X

### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## [0.1.28] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.27] (2021-04-07)
### Changed
* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

## [0.1.26] (2021-03-22)
### Added
* `--color <when>`
* some contents to `--help`

### Changed
* update depend: regex v1.4.5: fixes stack overflows

## [0.1.25] (2021-03-14)
### Changed
* update crate: regex: fix memory leak

## [0.1.24] (2021-03-08)
### Changed
* update crate: runnel
* update crate: rustc_version ("0.3")

## [0.1.23] (2021-03-08)
### Changed
* update crate: runnel
* rename file: xtask/src/cmd.txt to xtask/src/aki-gsub-cmd.txt
* cleanup src/main.rs and build.rs

## [0.1.22] (2021-03-02)
### Added
* some documents

### Changed
* change option: `-e, --expression` to `-e, --exp`
* update crate: flood-tide-gen

## [0.1.21] (2021-02-22)
### Added
* more doc

### Changed
* update crate: runnel, flood-tide-gen

### Fixed
* bug: add flush() on finish.

## [0.1.20] (2021-02-14)
### Added
* doc

### Changed
* update crate runnel
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

## [0.1.19] (2021-02-07)
### Changed
* update crates flood-tide-gen

## [0.1.18] (2021-02-05)
### Changed
* update crates for runnel

## [0.1.17] (2021-02-05)
### Fixed
* bug: README.md

## [0.1.16] (2021-02-05)
### Changed
* initial github

## 0.1.15 (2021-02-05)
### Added
* import crate exec-target from local, for test.

## 0.1.14 (2021-01-31)
### Changed
* change AppError to anyhow::Error
* change conf parser to flood-tied and flood-tied-gen
* some refactoring

## 0.1.13 (2021-01-24)
### Added
* `matches!()` macro support before rustc 1.42.0
* `cfg(has_fat_stdout)` and test support before rustc 1.44.0

## 0.1.12 (2021-01-24)
### Added
* add `pipeio`

### Changed
* rename `streamio` to `runnel`

## 0.1.11 (2021-01-22)
### Changed
* refactoring stream module

## 0.1.10 (2021-01-19)
### Added
* add tests with stream module

## 0.1.9 (2021-01-17)
### Added
* add xtask
* add stream module

### Changed
* change optpa_util_1 to flood-tide

## 0.1.8 (2020-12-29)
### Changed
* update crates

### Removed
* remove optpaerr-1

## 0.1.7 (2020-11-17)
### Added
* add `README.md`, `COPYING`, `LICENSE-APACHE`, `LICENSE-MIT`

### Changed
* change optpa_util to optpa_util_1

### Fixed
* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster

## 0.1.6 (2020-08-09)
### Added
* add support `cargo deb`

### Changed
* update crates

## 0.1.5 (2020-05-10)
### Changed
* change edition 2015 to 2018.
* update crates

## 0.1.4 (2020-03-30)
### Added
* add support broken pipe and test

### Changed
* update crates

## 0.1.3 (2019-04-14)
### Added
* add support std::alloc

### Changed
* update crates

## 0.1.2 (2018-05-04)
### Added
* add support cfg(has_global_allocator)

### Changed
* update crates

## 0.1.1 (2018-03-22)
### Added
* add support broken pipe
* a lot of things

### Changed
* update crates

## 0.1.0 (2017-12-12)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-gsub/compare/v0.1.35..HEAD
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
