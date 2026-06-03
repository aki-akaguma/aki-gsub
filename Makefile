
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

fetch:
	cargo fetch

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	MIRIFLAGS=-Zmiri-disable-isolation cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.* *.log *.tmp *.profraw

clippy:
	cargo clippy --offline --tests --workspace -- -W clippy::uninlined_format_args

fmt:
	cargo fmt

doc:
	cargo doc

tarpaulin:
	#cargo tarpaulin --offline --engine llvm --out html --output-dir ./target
	#cargo tarpaulin --offline --engine ptrace --out lcov --output-dir ./target
	cargo tarpaulin --offline --follow-exec --engine ptrace --out lcov --output-dir ./target
	genhtml -o target/lcov --demangle-cpp target/lcov.info

gen-src-cmd:
	cargo xtask gen-src-cmd gen-src-cmd
