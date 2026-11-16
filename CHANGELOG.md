## [Unreleased]

- Update algorithm definitions and generated Rust stemmers to Snowball 3.2.0.
- Synchronize the Rust runtime with Snowball 3.2.0.
- Add `Algorithm::EarlyModernEnglish` (39 supported algorithms in total).
- Add a pinned regeneration script and an optional full upstream vocabulary test.
- Stemming results may change; applications with persisted stemmed indexes should
  rebuild affected indexes when adopting this update.

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
