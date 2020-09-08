SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: install-dependencies build-libdiceroller-ios
	xcodebuild build \
            -project DiceRoller.xcodeproj \
            -scheme DiceRoller \
            -destination 'platform=iOS Simulator,name=iPhone 11' \
            | xcpretty

.PHONY: clean
clean: install-dependencies
	xcodebuild clean \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11' \
		| xcpretty

.PHONY: test
test: install-dependencies build-libdiceroller-ios
	xcodebuild test \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11' \
		| xcpretty

.PHONY: build-libdiceroller-ios
build-libdiceroller-ios:
	cd ../../platform-wrappers/ios && make build

.PHONY: install-dependencies
install-dependencies:
	bundle install
