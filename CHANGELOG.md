# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased][unreleased]
- Nothing yet.
## 0.3.2 - 2015-07-05
### Changed
- Encoder now should return proper error types.

## 0.3.1 - 2015-06-28
### Changed
- Stabilizing enum serialization/deserialization. Now every enum is serialized as [int, [args...]].
- Suppressed some warnings appeared on updated compiler.

## 0.3.0 - 2015-06-25
### Added
- Enum serialization/deserialization.

## 0.2.2 - 2015-06-15
### Changed
- Minor integer decoding performance tweaking.

## 0.2.1 - 2015-05-30
### Added
 - Benchmarking module.

### Changed
- Increased string decoding performance by ~30 times.
- Exported `read_value` function to the `rmp::decode` module.
- Exported `Value` struct to the root crate namespace.

## 0.2.0 - 2015-05-27
### Added
- Introducing a `Value` algebraic data type, which represents an owning MessagePack object. It can
  be found in `rmp::value` module.
- The Value ADT encoding and decoding functions.
- Low-level ext type decoders.

## 0.1.1 - 2015-05-18
### Changed
- Added documentation and repository site in Cargo.toml.
- Added keywords to ease searching using crates.io.

## 0.1.0 - 2015-05-15
### Added
- Initial commit.
- This CHANGELOG file to hopefully serve as an evolving example of a standardized open source
  project CHANGELOG.
