SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build:
	cd common/lib-dice-roller && make build
	cd platform-wrappers/ios && make build
	cd applications/ios && make build

.PHONY: clean
clean:
	cd common/lib-dice-roller && make clean
	cd platform-wrappers/ios && make clean
	cd applications/ios && make clean

.PHONY: test
test:
	cd common/lib-dice-roller && make test
	cd platform-wrappers/ios && make test
	cd applications/ios && make test
