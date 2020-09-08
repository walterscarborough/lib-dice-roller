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
