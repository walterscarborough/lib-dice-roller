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
	cd platform-wrappers/ios && make build
	cd applications/ios && make build

.PHONY: build-android
build-android:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-android-dependencies
	cd common/lib-dice-roller && make build-android
	cd platform-wrappers/android && make build

.PHONY: clean
clean:
	cd common/lib-dice-roller && make clean
	cd platform-wrappers/ios && make clean
	cd applications/ios && make clean

.PHONY: test-ios
test-ios:
	cd common/lib-dice-roller && make install-general-dependencies
	cd common/lib-dice-roller && make install-ios-dependencies
	cd common/lib-dice-roller && make test
	cd platform-wrappers/ios && make test
	cd applications/ios && make test
