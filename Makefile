SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build-ios
build-ios:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-ios-dependencies
	cd common/lib-dice-roller && make build-ios
	cd platform-adapters/ios && make build
	cd applications/ios && make build

.PHONY: build-android
build-android:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-android-dependencies
	cd common/lib-dice-roller && make build-android
	cd platform-adapters/android && make build

.PHONY: clean
clean:
	cd common/lib-dice-roller && make clean
	cd platform-adapters/ios && make clean
	cd applications/ios && make clean

.PHONY: test-ios
test-ios:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-ios-dependencies
	cd common/lib-dice-roller && make test
	cd platform-adapters/ios && make test
	cd applications/ios && make test

.PHONY: test-android
test-android:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-android-dependencies
	cd common/lib-dice-roller && make build-android
	cd platform-adapters/android && make test
