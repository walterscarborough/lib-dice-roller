SHELL := bash
.ONESHELL:
.SHELLFLAGS := -o errexit -o errtrace -o nounset -o pipefail -c
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build: build-libdiceroller-ios
	xcodebuild build \
            -project DiceRoller.xcodeproj \
            -scheme DiceRoller \
            -destination 'platform=iOS Simulator,name=iPhone 11'

.PHONY: clean
clean:
	xcodebuild clean \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11'

.PHONY: test
test: build-libdiceroller-ios
	xcodebuild test \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone 11'

.PHONY: build-libdiceroller-ios
build-libdiceroller-ios:
	cd ../../platform-wrappers/ios && make build
