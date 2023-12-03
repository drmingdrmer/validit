all: test check_all

check_all: lint fmt doc unused_dep typos

test:
	cargo test
	cargo +nightly test --features backtrace

bench:
	cargo bench --features bench

fmt:
	cargo +nightly fmt

fix:
	cargo fix --allow-staged

doc:
	RUSTDOCFLAGS="-D warnings" cargo +nightly doc --document-private-items --all --all-features --no-deps

check_missing_doc:
	# Warn about missing doc for public API
	RUSTDOCFLAGS="-W missing_docs" cargo +nightly doc --all --all-features --no-deps

lint:
	cargo +nightly fmt
	cargo +nightly clippy --no-deps --all-targets -- -D warnings
	# Bug: clippy --all-targets reports false warning about unused dep in
	# `[dev-dependencies]`:
	# https://github.com/rust-lang/rust/issues/72686#issuecomment-635539688
	# Thus we only check unused deps for lib
	RUSTFLAGS=-Wunused-crate-dependencies cargo +nightly clippy --no-deps  --lib -- -D warnings

unused_dep:
	cargo machete

typos:
	# cargo install typos-cli
	typos --write-changes
	# typos

clean:
	cargo clean

.PHONY: test fmt lint clean doc guide
