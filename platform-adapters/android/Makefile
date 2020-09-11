SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: build-libdiceroller-common
	./gradlew build

.PHONY: clean
clean:
	./gradlew clean

.PHONY: test
test: build-libdiceroller-common
	./gradlew test

.PHONY: build-libdiceroller-common
build-libdiceroller-common:
	cd ../../common/lib-dice-roller && make build-android
