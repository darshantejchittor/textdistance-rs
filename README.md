# textdistance-rs

A Rust port of [`life4/textdistance`](https://github.com/life4/textdistance) (Python),
implementing **15 string similarity and distance algorithms** across **5 algorithm families**.

Built for **Port Mortem 2026: Code Resurrection Hackathon** (Track D: Python → Rust).

## Highlights

- Ported 15 string distance/similarity algorithms from Python to Rust
- Covers 5 algorithm families
- Differentially verified against the original Python implementation using generated reference values
- 67 library unit tests + 1 differential verification test against Python-generated reference values
- Three implementation differences discovered and corrected during porting

## Scope

The original library contains 30+ independent, standalone algorithms. Rather than
attempt full coverage under a 72-hour deadline, we ported **15 algorithms
spanning 5 distinct algorithm families**, each fully unit-tested and
verified against the original Python library's actual output via
differential testing. See [`DECISIONS.md`](./DECISIONS.md) for the full
rationale on scope, trade-offs, and three concrete behavior corrections
we found and fixed along the way.

**In scope (15 algorithms):**

| Algorithm | Family |
|---|---|
| Levenshtein | Edit-based |
| Hamming | Edit-based |
| Damerau-Levenshtein | Edit-based |
| Jaccard | Token/multiset-based |
| Sorensen-Dice | Token/multiset-based |
| Cosine | Token/multiset-based |
| Overlap Coefficient | Token/multiset-based |
| Tversky Index | Token/multiset-based |
| Bag Distance | Token/multiset-based |
| Jaro | Phonetic / name-matching |
| Jaro-Winkler | Phonetic / name-matching |
| LCS (Longest Common Subsequence) | Sequence-based |
| Longest Common Substring | Sequence-based |
| Needleman-Wunsch | Sequence alignment |
| Q-gram (bigram) Distance | Chunk-based |

**Out of scope:** the remaining ~15 algorithms in the original library
(e.g. Smith-Waterman, Gotoh, Monge-Elkan, Ratcliff-Obershelp, compression-based
measures such as NCD). Not attempted due to time constraints and, for
several of these, meaningfully higher implementation risk under deadline
pressure — see `DECISIONS.md`.

## Project structure

This is both a **library crate** (`lib.rs`) and a small **binary** (`main.rs`)
in the same package:

```
textdistance-rs/
├── src/
│   ├── lib.rs                      # declares all 15 algorithm modules as `pub mod`
│   ├── main.rs                     # small demo binary + the differential test suite
│   ├── levenshtein.rs
│   ├── hamming.rs
│   ├── damerau.rs
│   ├── jaccard.rs
│   ├── sorensen.rs
│   ├── cosine.rs
│   ├── overlap.rs
│   ├── tversky.rs
│   ├── bag_distance.rs
│   ├── jaro.rs
│   ├── jaro_winkler.rs
│   ├── lcs.rs
│   ├── longest_common_substring.rs
│   ├── needleman_wunsch.rs
│   └── qgram.rs
├── scripts/
│   └── generate_reference.py       # generates ground-truth values from the real Python library
├── tests/
│   └── fixtures/
│       └── reference_values.json   # generated, not hand-written
├── DECISIONS.md                    # scope, trade-offs, and 3 verified findings
├── LICENSE                         # MIT
└── Cargo.toml
```

Each algorithm module has its own `#[cfg(test)] mod tests` block with
unit tests (edge cases, classic textbook examples, Unicode handling). The
**differential test** (`differential_tests::matches_python_reference`)
lives at the bottom of `main.rs`, since it needs to exercise every module
together against a single reference dataset.

## Build

```bash
cargo build --release
```

Single documented command, as required by the submission rules.

## Test

```bash
cargo test
```

Sample output:

```
running 68 tests
...
test differential_tests::matches_python_reference ... ok
...
test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

This runs, across both the library and binary targets:

- **67 library unit tests** — one module per algorithm, covering identical
  strings, empty strings, classic textbook examples, and
  algorithm-specific edge cases (transpositions, anagrams, Unicode, etc.)
- **1 differential verification test** (`matches_python_reference`,
  binary target only) — replays reference values generated directly from
  the original Python library and asserts our Rust output matches, across
  all 15 algorithms.
- **68 total tests** when running the binary target (67 + 1).

### Differential testing against the original Python library

The original Python implementation serves as the source of truth for behavioral verification. To
regenerate the reference data and re-verify against it yourself:

```bash
pip install textdistance numpy
python scripts/generate_reference.py
cargo test
```

`scripts/generate_reference.py` runs the real `textdistance` Python
package on a fixed set of test pairs (edge cases: empty strings, identical
strings, Unicode, classic algorithm textbook examples, transposition and
anagram cases) and writes the results to
`tests/fixtures/reference_values.json`. The Rust differential test then
reads that file and asserts every one of our 15 implementations produces
matching output for every case.

**Note:** `numpy` is a dependency of the original library's own
Needleman-Wunsch implementation (used by the *reference generator*
script), not of our Rust port.

**Claim, precisely stated:** our Rust implementations match the original
Python implementation for every case in this generated reference dataset.
This is distinct from (and in addition to) confirming the original
library's own test suite passes independently on its own — see
`DECISIONS.md` for the full, precise framing. For additional confidence,
we also confirmed the original Python library's own test suite passes
independently (430/430 tests), while our differential test verifies
behavioral equivalence for the generated reference dataset.

### Verification pipeline

```
Python textdistance
        │
        ▼
generate_reference.py
        │
        ▼
reference_values.json
        │
        ▼
Rust differential test
        │
        ▼
Behavior verified
```

### Verified implementation differences

Differential testing surfaced three real discrepancies between initial
assumptions and the original library's actual behavior, each resolved by
reading the original Python source directly:

1. **Hamming** — the original doesn't require equal-length strings; it
   uses `zip_longest` by default, counting length differences as extra
   mismatches.
2. **Needleman-Wunsch** — the original's default mismatch penalty is `0`,
   not `-1` as commonly assumed from textbook defaults.
3. **Jaro-Winkler** — the prefix boost is only applied when the underlying
   Jaro similarity exceeds `0.7`; below that threshold, no boost is added.

Full before/after details in [`DECISIONS.md`](./DECISIONS.md).

## Usage

As a library, from another crate or from within this one:

```rust
use textdistance_rs::levenshtein::levenshtein;
use textdistance_rs::jaro_winkler::jaro_winkler_similarity;
use textdistance_rs::needleman_wunsch::NeedlemanWunsch;
use textdistance_rs::tversky::Tversky;

fn main() {
    let d = levenshtein("kitten", "sitting");
    println!("{d}"); // 3

    let sim = jaro_winkler_similarity("DWAYNE", "DUANE");
    println!("{sim:.3}"); // ~0.84

    let nw = NeedlemanWunsch::default();
    println!("{}", nw.align_score("GATTACA", "GCATGCU"));

    let tv = Tversky::default(); // defaults reduce to Jaccard
    println!("{:.3}", tv.similarity("night", "nacht"));
}
```

## License

MIT — matching the permissive licensing of the original `life4/textdistance`
project, in compliance with the hackathon's open-source licensing
requirements.
