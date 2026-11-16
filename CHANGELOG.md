

## [1.1.0] - 2026-11-16

### Changed

- Update algorithm definitions and generated Rust stemmers to Snowball 3.2.0.
- Synchronize the Rust runtime with Snowball 3.2.0.
- Stemming results may change; applications with persisted stemmed indexes should
  rebuild affected indexes when adopting this update.

### Added

- Add `Algorithm::EarlyModernEnglish` (39 supported algorithms in total).
- Add a pinned regeneration script and an optional full upstream vocabulary test.

- All changes and additions contributed by @lauritsk in PR #2. Closes issue #1.

## [1.0.1] - 2026-03-11

### Changed

- README.md and LICENSE updated

## [1.0.0] - 2026-03-09

### Initial Release

- snowball_env.rs update to Snowball 3.0.0
- Stemmers update to Snowball 3.0.0
- Language support for
  - Arabic
  - Armenian
  - Basque
  - Catalan
  - Czech,
  - Danish
  - Dutch
  - DutchPorter
  - English
  - Esperanto
  - Estonian
  - Finnish
  - French
  - German
  - Greek
  - Hindi
  - Hungarian
  - Indonesian
  - Irish
  - Italian
  - Lithuanian
  - Lovins
  - Nepali
  - Norwegian
  - Persian
  - Polish
  - Porter
  - Portuguese
  - Romanian
  - Russian
  - Serbian
  - Sesotho
  - Spanish
  - Swedish
  - Tamil
  - Turkish
  - Ukrainian
  - Yiddish
