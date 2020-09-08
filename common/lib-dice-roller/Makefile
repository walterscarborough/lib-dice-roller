SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: install-rust-dependencies
	cargo lipo --release

.PHONY: clean
clean: install-rust-dependencies
	cargo clean

.PHONY: test
test: install-rust-dependencies
	cargo test

.PHONY: install-rust-dependencies
install-rust-dependencies:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	cargo install cargo-lipo
