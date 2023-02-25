
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	MIRIFLAGS=-Zmiri-disable-isolation cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.*
	@rm -f *.profraw

clippy:
	cargo clippy --offline --tests --workspace -- -W clippy::uninlined_format_args

fmt:
	cargo fmt

doc:
	cargo doc

tarpaulin:
	#cargo tarpaulin --offline --engine llvm --out html --output-dir ./target
	cargo tarpaulin --offline --engine llvm --out lcov --output-dir ./target
	#cargo tarpaulin --offline --engine ptrace --out lcov --output-dir ./target
	genhtml -o target/lcov --demangle-cpp target/lcov.info
