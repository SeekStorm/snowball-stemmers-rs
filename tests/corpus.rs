use snowball_stemmers_rs::{Algorithm, Stemmer};
use std::{env, fs, path::Path, process::Command};

const DATA_REVISION: &str = "019921c3a41c2b70063ee3feb215a00c8e594597";
const ALGORITHMS: &[(&str, Algorithm)] = &[
    ("arabic", Algorithm::Arabic),
    ("armenian", Algorithm::Armenian),
    ("basque", Algorithm::Basque),
    ("catalan", Algorithm::Catalan),
    ("czech", Algorithm::Czech),
    ("danish", Algorithm::Danish),
    ("dutch", Algorithm::Dutch),
    ("dutch_porter", Algorithm::DutchPorter),
    ("earlymodernenglish", Algorithm::EarlyModernEnglish),
    ("english", Algorithm::English),
    ("esperanto", Algorithm::Esperanto),
    ("estonian", Algorithm::Estonian),
    ("finnish", Algorithm::Finnish),
    ("french", Algorithm::French),
    ("german", Algorithm::German),
    ("greek", Algorithm::Greek),
    ("hindi", Algorithm::Hindi),
    ("hungarian", Algorithm::Hungarian),
    ("indonesian", Algorithm::Indonesian),
    ("irish", Algorithm::Irish),
    ("italian", Algorithm::Italian),
    ("lithuanian", Algorithm::Lithuanian),
    ("lovins", Algorithm::Lovins),
    ("nepali", Algorithm::Nepali),
    ("norwegian", Algorithm::Norwegian),
    ("persian", Algorithm::Persian),
    ("polish", Algorithm::Polish),
    ("porter", Algorithm::Porter),
    ("portuguese", Algorithm::Portuguese),
    ("romanian", Algorithm::Romanian),
    ("russian", Algorithm::Russian),
    ("serbian", Algorithm::Serbian),
    ("sesotho", Algorithm::Sesotho),
    ("spanish", Algorithm::Spanish),
    ("swedish", Algorithm::Swedish),
    ("tamil", Algorithm::Tamil),
    ("turkish", Algorithm::Turkish),
    ("ukrainian", Algorithm::Ukrainian),
    ("yiddish", Algorithm::Yiddish),
];

fn read_corpus(path: &Path) -> String {
    if path.is_file() {
        return fs::read_to_string(path).expect("read corpus file");
    }
    let compressed = path.with_extension("txt.gz");
    let output = Command::new("gzip")
        .arg("-cd")
        .arg(&compressed)
        .output()
        .expect("gzip is required for compressed corpus files");
    assert!(
        output.status.success(),
        "cannot read {}: {}",
        compressed.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("corpus must be UTF-8")
}

#[test]
#[ignore = "requires the pinned snowball-data checkout; see README"]
fn upstream_vocabulary() {
    let directory =
        env::var_os("SNOWBALL_DATA").expect("set SNOWBALL_DATA to the upstream data checkout");
    let directory = Path::new(&directory);
    let revision = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("read snowball-data revision");
    assert!(revision.status.success());
    assert_eq!(
        String::from_utf8(revision.stdout).unwrap().trim(),
        DATA_REVISION,
        "use the data revision documented in README"
    );
    let changes = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .unwrap();
    assert!(changes.status.success());
    assert!(
        changes.stdout.is_empty(),
        "snowball-data must have no tracked changes"
    );
    let mut total = 0;
    for &(name, algorithm) in ALGORITHMS {
        let words = read_corpus(&directory.join(name).join("voc.txt"));
        let expected = read_corpus(&directory.join(name).join("output.txt"));
        let words: Vec<_> = words.lines().collect();
        let expected: Vec<_> = expected.lines().collect();
        assert!(!words.is_empty(), "{name}: empty vocabulary");
        assert_eq!(
            words.len(),
            expected.len(),
            "{name}: corpus line counts differ"
        );
        let stemmer = Stemmer::create(algorithm);
        for (index, (&word, &stem)) in words.iter().zip(&expected).enumerate() {
            assert_eq!(
                stemmer.stem(word),
                stem,
                "{name}, line {}, input {word:?}",
                index + 1
            );
        }
        println!("{name}: {} words passed", words.len());
        total += words.len();
    }
    println!("{} algorithms, {total} words passed", ALGORITHMS.len());
}

#[test]
fn empty_input_for_every_algorithm() {
    for &(name, algorithm) in ALGORITHMS {
        assert_eq!(Stemmer::create(algorithm).stem(""), "", "{name}");
    }
}

#[test]
fn early_modern_english() {
    let stemmer = Stemmer::create(Algorithm::EarlyModernEnglish);
    for (word, expected) in [("lov'd", "love"), ("loveth", "love"), ("ſinging", "sing")] {
        assert_eq!(stemmer.stem(word), expected, "{word}");
    }
}
