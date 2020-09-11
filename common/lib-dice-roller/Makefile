SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build:
	cargo build --release

.PHONY: build-ios
build-ios:
	cargo lipo --release

.PHONY: build-android
build-android:
	cargo build --target i686-linux-android --release --features "enable-jni"
	cargo build --target armv7-linux-androideabi --release --features "enable-jni"
	cargo build --target aarch64-linux-android --release --features "enable-jni"

.PHONY: build-android-with-logging
build-android-with-logging:
	cargo build --target i686-linux-android --release --features "enable-jni enable-android-logging"

.PHONY: clean
clean:
	cargo clean

.PHONY: test
test:
	cargo test

.PHONY: install-general-dependencies
install-general-dependencies:
	brew bundle
	rustup update

.PHONY: install-ios-dependencies
install-ios-dependencies:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	cargo install cargo-lipo

.PHONY: install-android-dependencies
install-android-dependencies:
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
