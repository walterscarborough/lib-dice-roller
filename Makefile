SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build-ios
build-ios:
	cd common/dice-roller-ffi && make build
	cd platform-adapters/ios && make build
	cd applications/ios && make build

.PHONY: build-android
build-android:
	cd common/dice-roller-jni && make build
	cd platform-adapters/android && make build

.PHONY: clean-ios
clean-ios:
	cd common/dice-roller && make clean
	cd common/dice-roller-ffi && make clean
	cd platform-adapters/ios && make clean
	cd applications/ios && make clean

.PHONY: clean-android
clean-android:
	cd common/dice-roller && make clean
	cd common/dice-roller-jni && make clean
	cd platform-adapters/android && make clean

.PHONY: test-ios
test-ios:
	cd common/dice-roller-ffi && make test
	cd platform-adapters/ios && make test
	cd applications/ios && make test

.PHONY: test-android
test-android:
	cd common/dice-roller-jni && make test
	cd platform-adapters/android && make test
