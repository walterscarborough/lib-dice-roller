SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: install-dependencies
	wasm-pack build --release

.PHONY: clean
clean: install-dependencies
	cargo clean

.PHONY: test
test: install-dependencies
	wasm-pack test --node

.PHONY: install-dependencies
install-dependencies:
	brew bundle
	rustup update
	cargo install wasm-pack
