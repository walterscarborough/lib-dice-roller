SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: build-libdiceroller-jni
	./gradlew build

.PHONY: clean
clean:
	./gradlew clean

.PHONY: test
test: build-libdiceroller-jni
	./gradlew test

.PHONY: build-libdiceroller-jni
build-libdiceroller-jni:
	cd ../../common/dice-roller-jni && make build
