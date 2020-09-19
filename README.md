# LibDiceRoller 🎲

![Rust Pipeline](https://github.com/walterscarborough/lib-dice-roller/workflows/Rust%20Pipeline/badge.svg)

![Android Pipeline](https://github.com/walterscarborough/lib-dice-roller/workflows/Android%20Pipeline/badge.svg)

![iOS Pipeline](https://github.com/walterscarborough/lib-dice-roller/workflows/iOS%20Pipeline/badge.svg)

![Wasm Pipeline](https://github.com/walterscarborough/lib-dice-roller/workflows/Wasm%20Pipeline/badge.svg)

## Overview
Welcome to LibDiceRoller!

This project has two purposes: 
1. make a handy cross platform dice rolling library that safely and quickly runs on iOS, Android, the Web (Typescript / JavaScript), Linux, MacOS, and Windows
1. explore interesting design and communication patterns for portable software architectures

## Current Platform Compatibility
- [x] iOS
- [x] Android
- [ ] Web
- [ ] Linux
- [ ] MacOS
- [ ] Windows

## Architecture and Design
This project consists of the following components:
* a rust library that can be conditionally compiled to support the C FFI (foreign function interface), JNI (java native interface). Webassembly support for running on the web is currently in progress.
* platform adapters that allow the rust library to easily run on target platforms. For iOS and Android, this means small projects that wrap LibDiceRoller into a Xcode framework and Android library, respectively.
* a serialized data communication layer between the rust library and the platform adapters. It's currently using protobufs!

### Communication Layers
All communication between the rust layer and the platform adapters (e.g. iOS, Android, etc) is handled by passing serialized data as protobufs.
I chose a serialized data format because it's quite tedious to have to create a new FFI or JNI or other future platform data mapping whenever my data format needs to change - let alone keep them all in sync and up to date! 
With a serialized data format, I could encode/decode a data model through the same interface (or port/adapter if you're a hexagonal architecture fan) and still have the flexibility to easily change it without needing to update the communication layer directly.

#### Communication Layers: why use a serialized data format? 
After considering which serialized data formats would work well across multiple platforms, it came down to a decision between JSON, protobufs, or flatbuffers.
JSON would be the easiest to implement because JSON support is ubiquitous across every modern target.
However, it would also arguably be the most unsafe option for the long term because I'd have no way of knowing at compile time if I was parsing the current format of a given data model.

I experimented with using flatbuffers, and although they did work, swift support was still unstable in the current release as of early September 2020.
After trying out protobufs, I was pleasantly surprised to see that they just worked on iOS and Android.

## Setup

You will need the following:
* MacOS Catalina
* Homebrew
* Xcode 11.7
* Android Studio 4.0.1

### iOS Setup

*Build*
```bash
make build-ios
```

*Test*
```bash
make test-ios
```

*Clean*
```bash
make clean-ios
```

### Android Setup

The Android NDK will need to be installed and setup in order to build LibDiceRoller. 
You can do set it up with Android Studio, or by downloading it from the Google Android Developer site.

You will also need to let Rust know about it as follows:
* edit the file common/lib-dice-roller/cargo-config.toml and update the NDK paths to match its path on your system.
* put a copy of cargo-config.toml in a place where cargo can read it: `mkdir -p common/lib-dice-roller/.cargo && cp common/lib-dice-roller/cargo-config.toml common/lib-dice-roller/.cargo/config.toml`

Feel free to take a look at the Android pipeline if you get stuck or just want to see a working example of how to set it up: `.github/workflows/android-pipeline.yml`

*Build*
```bash
make build-android
```

*Test*
```bash
make test-android
```

*Clean*
```bash
make clean-android
```

## Thoughts? Ideas?
Please share any thoughts, ideas, or suggestions that you have.

## License
MIT License Copyright (c) 2020 Walter Scarborough
