SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: build-libdiceroller-jni install-dependencies
	./gradlew build

.PHONY: clean
clean: install-dependencies
	./gradlew clean

.PHONY: test
test: build-libdiceroller-jni install-dependencies
	./gradlew test

.PHONY: build-libdiceroller-jni
build-libdiceroller-jni:
	cd ../../common/dice-roller-jni && make build

.PHONY: install-dependencies
install-dependencies:
	brew bundle
