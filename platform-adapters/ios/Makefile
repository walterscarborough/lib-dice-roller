SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: build-libdiceroller-common install-dependencies generate-protobufs
	xcodebuild build \
            -project LibDiceRoller.xcodeproj \
            -scheme LibDiceRoller \
            -destination 'platform=iOS Simulator,name=iPhone 11' \
            | xcpretty

.PHONY: clean
clean: install-dependencies
	xcodebuild clean \
		-project LibDiceRoller.xcodeproj \
		-scheme LibDiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11' \
		| xcpretty

.PHONY: test
test: build-libdiceroller-common install-dependencies generate-protobufs
	xcodebuild test \
		-project LibDiceRoller.xcodeproj \
		-scheme LibDiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11' \
		| xcpretty

.PHONY: generate-protobufs
generate-protobufs:
	./scripts/generate-protobufs.sh

.PHONY: build-libdiceroller-common
build-libdiceroller-common:
	cd ../../common/lib-dice-roller && make build-ios

.PHONY: install-dependencies
install-dependencies:
	brew bundle
	bundle install
