SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build:
	wasm-pack build --release

.PHONY: clean
clean:
	cargo clean

.PHONY: test
test:
	wasm-pack test --node

.PHONY: install-dependencies
install-dependencies:
	brew bundle
	rustup update
	cargo install wasm-pack
