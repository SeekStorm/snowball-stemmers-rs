//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! rust-stemmers = "^1.0"
//! ```
//!
//! ```rust
//! extern crate snowball_stemmers_rs;
//!
//! use snowball_stemmers_rs::{Algorithm, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::create(Algorithm::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

mod snowball;

use snowball::algorithms;
use snowball::SnowballEnv;

/// Enum of all supported algorithms.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Czech,
    Danish,
    Dutch,
    DutchPorter,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Lovins,
    Nepali,
    Norwegian,
    Persian,
    Polish,
    Porter,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Sesotho,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Ukrainian,
    Yiddish,
}

/// Interface around the Snowball stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Arabic => Stemmer {
                stemmer: algorithms::arabic::stem,
            },
            Algorithm::Armenian => Stemmer {
                stemmer: algorithms::armenian::stem,
            },
            Algorithm::Basque => Stemmer {
                stemmer: algorithms::basque::stem,
            },
            Algorithm::Catalan => Stemmer {
                stemmer: algorithms::catalan::stem,
            },
            Algorithm::Czech => Stemmer {
                stemmer: algorithms::czech::stem,
            },
            Algorithm::Danish => Stemmer {
                stemmer: algorithms::danish::stem,
            },
            Algorithm::Dutch => Stemmer {
                stemmer: algorithms::dutch::stem,
            },
            Algorithm::DutchPorter => Stemmer {
                stemmer: algorithms::dutch_porter::stem,
            },
            Algorithm::English => Stemmer {
                stemmer: algorithms::english::stem,
            },
            Algorithm::Esperanto => Stemmer {
                stemmer: algorithms::esperanto::stem,
            },
            Algorithm::Estonian => Stemmer {
                stemmer: algorithms::estonian::stem,
            },
            Algorithm::Finnish => Stemmer {
                stemmer: algorithms::finnish::stem,
            },
            Algorithm::French => Stemmer {
                stemmer: algorithms::french::stem,
            },
            Algorithm::German => Stemmer {
                stemmer: algorithms::german::stem,
            },
            Algorithm::Greek => Stemmer {
                stemmer: algorithms::greek::stem,
            },
            Algorithm::Hindi => Stemmer {
                stemmer: algorithms::hindi::stem,
            },
            Algorithm::Hungarian => Stemmer {
                stemmer: algorithms::hungarian::stem,
            },
            Algorithm::Indonesian => Stemmer {
                stemmer: algorithms::indonesian::stem,
            },
            Algorithm::Irish => Stemmer {
                stemmer: algorithms::irish::stem,
            },
            Algorithm::Italian => Stemmer {
                stemmer: algorithms::italian::stem,
            },
            Algorithm::Lithuanian => Stemmer {
                stemmer: algorithms::lithuanian::stem,
            },
            Algorithm::Lovins => Stemmer {
                stemmer: algorithms::lovins::stem,
            },
            Algorithm::Nepali => Stemmer {
                stemmer: algorithms::nepali::stem,
            },
            Algorithm::Norwegian => Stemmer {
                stemmer: algorithms::norwegian::stem,
            },
            Algorithm::Persian => Stemmer {
                stemmer: algorithms::persian::stem,
            },
            Algorithm::Polish => Stemmer {
                stemmer: algorithms::polish::stem,
            },
            Algorithm::Porter => Stemmer {
                stemmer: algorithms::porter::stem,
            },
            Algorithm::Portuguese => Stemmer {
                stemmer: algorithms::portuguese::stem,
            },
            Algorithm::Romanian => Stemmer {
                stemmer: algorithms::romanian::stem,
            },
            Algorithm::Russian => Stemmer {
                stemmer: algorithms::russian::stem,
            },
            Algorithm::Serbian => Stemmer {
                stemmer: algorithms::serbian::stem,
            },
            Algorithm::Sesotho => Stemmer {
                stemmer: algorithms::sesotho::stem,
            },
            Algorithm::Spanish => Stemmer {
                stemmer: algorithms::spanish::stem,
            },
            Algorithm::Swedish => Stemmer {
                stemmer: algorithms::swedish::stem,
            },
            Algorithm::Tamil => Stemmer {
                stemmer: algorithms::tamil::stem,
            },
            Algorithm::Turkish => Stemmer {
                stemmer: algorithms::turkish::stem,
            },
            Algorithm::Ukrainian => Stemmer {
                stemmer: algorithms::ukrainian::stem,
            },
            Algorithm::Yiddish => Stemmer {
                stemmer: algorithms::yiddish::stem,
            },
        }
    }

    /// Stem a single word, the input is expected to be in lowercase.
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}

#[cfg(test)]
mod tests {
    use super::{Algorithm, Stemmer};

    #[test]
    fn english_test() {
        let vocabulary = vec![
            ("fishing", "fish"),
            ("running", "run"),
            ("quickly", "quick"),
            ("connection", "connect"),
            ("cared", "care"),
            ("jumped", "jump"),
            ("skies", "sky"),
        ];
        let stemmer = Stemmer::create(Algorithm::English);

        for (voc, res) in vocabulary {
            assert_eq!(stemmer.stem(voc), res);
        }
    }

    #[test]
    fn polish_test() {
        let vocabulary = vec![
            ("samochód", "samochód"),
            ("samochodu", "samochod"),
            ("samochodowi", "samochod"),
            ("samochodem", "samochod"),
            ("samochodzie", "samochodz"),
            ("samochody", "samochod"),
            ("samochodów", "samochod"),
            ("samochodom", "samochod"),
            ("samochodami", "samochod"),
            ("samochodach", "samochod"),
            ("stołowi", "stoł"),
            ("słoniowi", "słon"),
        ];
        let stemmer = Stemmer::create(Algorithm::Polish);

        for (voc, res) in vocabulary {
            assert_eq!(stemmer.stem(voc), res);
        }
    }
}
