# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add `Deserializer::read_bytes_into` to decode a byte array into a reusable buffer.

### Changed

- Pre-allocate for short byte arrays in `read_bytes` to avoid repeated reallocation.
- Bulk-copy byte chunks in `put_slice` for both normal and reverse modes instead of putting byte by byte.

### Fixed

- Return `Error::Eof` instead of panicking when the input is truncated.

## [0.2.0] - 2023-05-16

### Changed

- `Decimal::NaN` is now encoded as larger than `+Infinity`.

## [0.1.1] - 2023-04-12

### Added

- Add serialize and deserialize for `i128` and `u128`.

## [0.1.0] - 2022-11-29

- Initial release.
