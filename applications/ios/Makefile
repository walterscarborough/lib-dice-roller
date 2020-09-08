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
            -destination 'platform=iOS Simulator,name=iPhone SE (2nd generation)'

.PHONY: clean
clean:
	xcodebuild clean \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone SE (2nd generation)'

.PHONY: test
test: build-libdiceroller-ios
	xcodebuild test \
		-project DiceRoller.xcodeproj \
		-scheme DiceRoller \
		-destination 'platform=iOS Simulator,name=iPhone SE (2nd generation)'

.PHONY: build-libdiceroller-ios
build-libdiceroller-ios:
	cd ../../platform-wrappers/ios && make build
