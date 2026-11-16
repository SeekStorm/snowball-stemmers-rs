#!/usr/bin/env python3
"""Regenerate the vendored stemmers from an unmodified Snowball 3.2.0 checkout."""
import argparse
from pathlib import Path
import shutil
import subprocess

SNOWBALL_REVISION = "6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("snowball", type=Path, help="path to the Snowball v3.2.0 checkout")
    args = parser.parse_args()
    upstream = args.snowball.resolve()
    revision = subprocess.check_output(
        ["git", "-C", str(upstream), "rev-parse", "HEAD"], text=True
    ).strip()
    if revision != SNOWBALL_REVISION:
        parser.error(f"expected Snowball revision {SNOWBALL_REVISION}, got {revision}")
    changes = subprocess.check_output(
        ["git", "-C", str(upstream), "status", "--porcelain", "--untracked-files=no"],
        text=True,
    )
    if changes:
        parser.error("Snowball checkout has tracked changes; use an unmodified checkout")
    # Check required tools before modifying vendored files.
    if shutil.which("rustfmt") is None:
        parser.error("rustfmt is required (rustup component add rustfmt)")
    subprocess.run(["make", "-C", str(upstream), "snowball"], check=True)
    root = Path(__file__).resolve().parents[1]
    generated = root / "src/snowball/algorithms"
    sources = sorted((upstream / "algorithms").glob("*.sbl"))
    rust_files = []
    for source in sources:
        shutil.copyfile(source, root / "algorithms" / source.name)
        target = generated / source.stem
        subprocess.run(
            [str(upstream / "snowball"), str(source), "-rust", "-o", str(target)],
            check=True,
        )
        rust_file = target.with_suffix(".rs")
        rust_file.write_text(
            rust_file.read_text(encoding="utf-8").replace("use snowball::", "use crate::snowball::"),
            encoding="utf-8",
        )
        rust_files.append(rust_file)
    modules = generated / "mod.rs"
    modules.write_text("".join(f"pub mod {source.stem};\n" for source in sources), encoding="utf-8")
    rust_files.append(modules)
    for name in ("snowball_env.rs", "among.rs"):
        source = upstream / "rust/src/snowball" / name
        target = root / "src/snowball" / name
        target.write_text(
            source.read_text(encoding="utf-8").replace("use snowball::", "use crate::snowball::"),
            encoding="utf-8",
        )
        rust_files.append(target)
    subprocess.run(["rustfmt", "--edition", "2024", *map(str, rust_files)], check=True)
    print(f"Regenerated {len(sources)} stemmers with Snowball 3.2.0")


if __name__ == "__main__":
    main()
