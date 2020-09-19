SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build:
	cargo lipo --release

.PHONY: clean
clean:
	cargo clean

.PHONY: test
test:
	cargo test

.PHONY: install-dependencies
install-dependencies:
	brew bundle
	rustup update
	rustup target add aarch64-apple-ios x86_64-apple-ios
	cargo install cargo-lipo
