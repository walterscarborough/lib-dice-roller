SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: install-dependencies
	cargo build --target i686-linux-android --release --features "enable-jni"
	cargo build --target armv7-linux-androideabi --release --features "enable-jni"
	cargo build --target aarch64-linux-android --release --features "enable-jni"

.PHONY: build-with-logging
build-with-logging: install-dependencies
	cargo build --target i686-linux-android --release --features "enable-jni enable-android-logging"

.PHONY: clean
clean: install-dependencies
	cargo clean

.PHONY: test
test: install-dependencies
	cargo test

.PHONY: install-dependencies
install-dependencies:
	brew bundle
	rustup update
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
