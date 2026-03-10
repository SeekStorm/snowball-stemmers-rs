# snowball_stemmers_rs

[![Crates.io](https://img.shields.io/crates/v/snowball_stemmers_rs.svg)](https://crates.io/crates/snowball_stemmers_rs)
[![Downloads](https://img.shields.io/crates/d/snowball_stemmers_rs.svg?style=flat-square)](https://crates.io/crates/snowball_stemmers_rs)
[![Documentation](https://docs.rs/snowball_stemmers_rs/badge.svg)](https://docs.rs/snowball_stemmers_rs)
[![MIT License](https://img.shields.io/github/license/SeekStorm/snowball-stemmers-rs.svg)](https://github.com/SeekStorm/snowball-stemmers-rs/blob/main/LICENSE)


Snowball_stemmers_rs implements the stemmer algorithms from the [snowball project](http://snowballstem.org/) which are compiled to Rust using the rust-backend of the [snowball compiler](https://github.com/snowballstem/snowball).

# Supported Languages/Algorithms

-   Arabic
-   Armenian
-   Basque
-   Catalan
-   Czech,
-   Danish
-   Dutch
-   DutchPorter
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
`git clone https://github.com/snowballstem/snowball.git`

Compile the compiler:
Navigate to the root and run `make`. This produces the snowball executable. 

### Generate Rust Code for the additional stemmer language 
The **algorithm definition** is located at `algorithms/additional_language.sbl`. Use the compiler with the -rust flag to generate the Rust implementation. 

Command:
`./snowball algorithms/additional_language.sbl -rust -o additional_language`


