# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- HTTP client dependency updated.

## [0.2.0] - 2026-11-06

### Added
- Client identifies itself with its name and version; `KwtSms::with_app_name()` for your app.

### Changed
- Package author and license holder are now kwtSMS.
- `send_with_retry_wait()` added for an explicit retry wait.

## [0.1.10] - 2026-03-15

### Added

- Password masked in debug output.

### Changed

- Bug fixes.

## [0.1.9] - 2026-03-15

### Added

- Country-specific phone validation (82 countries).
- Trunk prefix handling.

### Changed

- Embedded CLI removed (replaced by standalone [kwtsms-cli](https://github.com/boxlinknet/kwtsms-cli)).

## [0.1.8] - 2026-03-06

### Changed

- Maintenance.

## [0.1.7] - 2026-03-06

### Added

- Production OTP example.

## [0.1.6] - 2026-03-06

### Changed

- Maintenance.

## [0.1.5] - 2026-03-06

### Added

- Full README and CONTRIBUTING guides.

### Changed

- Maintenance.

## [0.1.4] - 2026-03-06

### Added

- README badges.

## [0.1.3] - 2026-03-06

### Fixed

- Bug fixes.

## [0.1.2] - 2026-03-06

### Fixed

- Bug fixes.

## [0.1.1] - 2026-03-06

### Changed

- Maintenance.

## [0.1.0] - 2026-03-06

### Added

- Initial release.
- Full API coverage: send, balance, validate, sender IDs, coverage, status, delivery reports.
- Phone number normalization and message cleaning.
- Bulk send with auto-batching and retry.
- Error codes mapped to action messages.
- `.env` support, JSONL logging and cached balance.
- Test suite.
