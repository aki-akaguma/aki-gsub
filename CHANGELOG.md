aki-gsub TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.29 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

0.1.28 (2021-04-19)
=====

* update depends: flood-tide-gen(0.1.10)

0.1.27 (2021-04-07)
=====

* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

0.1.26 (2021-03-22)
=====

* update depend: regex v1.4.5: fixes stack overflows
* add --color <when>
* add some contents to --help

0.1.25 (2021-03-14)
=====

* update crate: regex: fix memory leak

0.1.24 (2021-03-08)
=====

* update crate: runnel
* update crate: rustc_version ("0.3")

0.1.23 (2021-03-08)
=====

* update crate: runnel
* rename file: xtask/src/cmd.txt to xtask/src/aki-gsub-cmd.txt
* cleanup src/main.rs and build.rs

0.1.22 (2021-03-02)
=====

* change option: '-e, --expression' to '-e, --exp'
* update crate: flood-tide-gen
* add some documents

0.1.21 (2021-02-22)
=====

* fix bug: add flush() on finish.
* update crate: runnel, flood-tide-gen
* add more doc

0.1.20 (2021-02-14)
=====

* update crate runnel
* add doc
* rename section "AAA-admin" to "AAA-text" of package.metadata.deb

0.1.19 (2021-02-07)
=====

* update crates flood-tide-gen

0.1.18 (2021-02-05)
=====

* update crates for runnel

0.1.17 (2021-02-05)
=====

* fix README.md

0.1.16 (2021-02-05)
=====

* initial github

0.1.15 (2021-02-05)
=====

* import crate exec-target from local, for test.

0.1.14 (2021-01-31)
=====

* change AppError to anyhow::Error
* change conf parser to flood-tied and flood-tied-gen
* some refactoring

0.1.13 (2021-01-24)
=====

* add matches!() macro support before rustc 1.42.0
* add cfg(has_fat_stdout) and test support before rustc 1.44.0

0.1.12 (2021-01-24)
=====

* add pipeio to streamio crate
* rename streamio to runnel

0.1.11 (2021-01-22)
=====

* refactoring stream module

0.1.10 (2021-01-19)
=====

* add tests with stream module

0.1.9 (2021-01-17)
=====

* add xtask
* add stream module
* change optpa_util_1 to flood-tide

0.1.8 (2020-12-29)
=====

* update crates
* remove optpaerr-1

0.1.7 (2020-11-17)
=====

* fix old version: rustc_version(=0.2.3), v0.3.0 is not compile new semver on deb10-buster
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT
* change optpa_util to optpa_util_1

0.1.6 (2020-08-09)
=====

* add support cargo deb
* update crates

0.1.5 (2020-05-10)
=====

* change edition 2015 to 2018.
* update crates

0.1.4 (2020-03-30)
=====

* add support broken pipe and test
* update crates

0.1.3 (2019-04-14)
=====

* add support std::alloc
* update crates

0.1.2 (2018-05-04)
=====

* add support cfg(has_global_allocator)
* update crates

0.1.1 (2018-03-22)
=====

* add support broken pipe
* update crates
* a lot of things

0.1.0 (2017-12-12)
=====
first commit
