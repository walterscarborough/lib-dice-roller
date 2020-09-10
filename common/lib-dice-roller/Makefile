SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: install-general-dependencies
	cargo lipo --release

.PHONY: build-android
build-android: install-android-dependencies
	cargo build --target i686-linux-android --release --features "enable-jni"

.PHONY: build-android-with-logging
build-android-with-logging: install-android-dependencies
	cargo build --target i686-linux-android --release --features "enable-jni enable-android-logging"

.PHONY: clean
clean: install-general-dependencies
	cargo clean

.PHONY: test
test: install-general-dependencies
	cargo test

.PHONY: install-general-dependencies
install-general-dependencies:
	brew bundle
	rustup update
	rustup target add aarch64-apple-ios x86_64-apple-ios
	cargo install cargo-lipo

.PHONY: install-android-dependencies
install-android-dependencies: install-general-dependencies
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
