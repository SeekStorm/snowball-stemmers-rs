# snowball_stemmers_rs

[![Crates.io](https://img.shields.io/crates/v/snowball_stemmers_rs.svg)](https://crates.io/crates/snowball_stemmers_rs)
[![Downloads](https://img.shields.io/crates/d/snowball_stemmers_rs.svg?style=flat-square)](https://crates.io/crates/snowball_stemmers_rs)
[![Documentation](https://docs.rs/snowball_stemmers_rs/badge.svg)](https://docs.rs/snowball_stemmers_rs)
[![MIT License](https://img.shields.io/github/license/SeekStorm/snowball-stemmers-rs.svg)](https://github.com/SeekStorm/snowball-stemmers-rs/blob/main/LICENSE)


Snowball_stemmers_rs implements all 39 algorithms from [Snowball 3.2.0](https://snowballstem.org/), compiled to Rust using the Rust backend of the [Snowball compiler](https://github.com/snowballstem/snowball).

# Supported Languages/Algorithms

-   Arabic
-   Armenian
-   Basque
-   Catalan
-   Czech
-   Danish
-   Dutch
-   DutchPorter
-   EarlyModernEnglish
-   English
-   Esperanto
-   Estonian
-   Finnish
-   French
-   German
-   Greek
-   Hindi
-   Hungarian
-   Indonesian
-   Irish
-   Italian
-   Lithuanian
-   Lovins
-   Nepali
-   Norwegian
-   Persian
-   Polish
-   Porter
-   Portuguese
-   Romanian
-   Russian
-   Serbian
-   Sesotho
-   Spanish
-   Swedish
-   Tamil
-   Turkish
-   Ukrainian
-   Yiddish

# Usage

```rust
extern crate snowball_stemmers_rs;
use snowball_stemmers_rs::{Algorithm, Stemmer};

// Create a stemmer for the english language
let en_stemmer = Stemmer::create(Algorithm::English);

// Stem the word "fishing"
// All algorithms expect their input in lowercase.
assert_eq!(en_stemmer.stem("fishing"), "fish");
```

# Add an additional Language

To create an additional stemmer language for Rust from the official Snowball repository, you must use the Snowball compiler (snowball) to translate the language algorithm definition into Rust source code. 

### Build the Snowball Compiler 

The Snowball compiler is written in ISO C. You first need to build it to generate the executable that performs the translation. 

Clone the repository:
`git clone --branch v3.2.0 --depth 1 https://github.com/snowballstem/snowball.git`

Compile the compiler:
Navigate to the root and run `make`. This produces the snowball executable. 

### Generate Rust Code for the additional stemmer language 
The **algorithm definition** is located at `algorithms/additional_language.sbl`. Use the compiler with the -rust flag to generate the Rust implementation. 

Command:
`./snowball algorithms/additional_language.sbl -rust -o additional_language`

When integrating the output into this crate, change `use snowball::` imports to
`use crate::snowball::` and register the module and public `Algorithm` variant.

# Updating the vendored stemmers

The algorithm definitions, Rust runtime, and generated implementations are pinned
to Snowball **3.2.0**, commit `6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11`.
Regeneration requires Git, a C compiler, Make, Python 3, and rustfmt:

```sh
git clone --branch v3.2.0 --depth 1 https://github.com/snowballstem/snowball.git ../snowball
python3 scripts/generate.py ../snowball
cargo test
cargo fmt --check
```

The generator checks the upstream revision, copies the algorithm definitions and
runtime, generates all 39 algorithms, adapts imports for this crate, and formats
the generated Rust. When adding an algorithm, also add it to `Algorithm` and
`Stemmer::create` in `src/lib.rs` and to the corpus test's algorithm list.

To run the full upstream vocabulary checks, use the snowball-data revision from
2026-10-16, immediately before the Snowball 3.2.0 release. The corpus stays outside
this repository; some large files require `gzip` to read.

```sh
git clone https://github.com/snowballstem/snowball-data.git ../snowball-data
git -C ../snowball-data checkout 019921c3a41c2b70063ee3feb215a00c8e594597
SNOWBALL_DATA=../snowball-data cargo test --release --test corpus -- --ignored --nocapture
```

The test verifies this data revision and compares every word with its expected
stem for every supported algorithm. Normal `cargo test` runs the small regression
tests without requiring an external corpus or network access.

Stemming results can change when upgrading Snowball. Applications that persist
stemmed search indexes should rebuild affected indexes so that indexed terms and
queries use the same stemming rules.
