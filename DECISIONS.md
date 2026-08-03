# Architectural Decisions

Port of [`life4/textdistance`](https://github.com/life4/textdistance) (Python)
to Rust — Port Mortem 2026: Code Resurrection Hackathon (Track D: Python → Rust).

## Scope

`life4/textdistance` contains 30+ independent algorithms behind a shared
interface. Rather than attempt full coverage under a 72-hour deadline, we
ported **15 algorithms spanning 5 distinct families**:

- **Edit-based:** Levenshtein, Hamming, Damerau-Levenshtein
- **Token/multiset-based:** Jaccard, Sorensen-Dice, Cosine, Overlap Coefficient, Tversky Index, Bag Distance
- **Phonetic / name-matching:** Jaro, Jaro-Winkler
- **Sequence-based:** LCS (subsequence), Longest Common Substring, Needleman-Wunsch
- **Chunk-based:** Q-gram (bigram) Distance

**Out of scope:** the remaining ~15 algorithms (e.g. Smith-Waterman, Gotoh,
Monge-Elkan, Ratcliff-Obershelp, compression-based measures such as NCD).
Not attempted due to time constraints and, for several of these, meaningfully
higher implementation risk under deadline pressure than what we chose to
prioritize.

This scoping decision follows guidance the organizers gave directly in
Discord: *"A well-scoped subset is fine. Full coverage is better, but only
if it is solid... A solid, proven subset is better than an unstable full
port."* We optimized for depth of verification over breadth of coverage.

## Verification methodology

Every ported algorithm is unit-tested (68 tests total covering identical
strings, empty strings, Unicode, and algorithm-specific edge cases), and
additionally verified against the original Python library through a
differential testing process:

1. `scripts/generate_reference.py` installs and runs the actual
   `life4/textdistance` Python package on a fixed set of test pairs
   (edge cases, Unicode, classic textbook examples, transposition/anagram
   cases), writing results to `tests/fixtures/reference_values.json`.
2. A Rust differential test (`matches_python_reference`) reads that file
   and asserts every one of our 15 implementations produces matching
   output for every generated case.

**Precise claim:** our Rust implementations match the original Python
implementation for every case in the generated reference dataset. This is
not a claim that we replayed the original library's full internal test
suite (which we separately confirmed passes 430/430 on its own, as
context that the library itself is a solid ground truth) — it's a claim
about our own generated, representative comparison set. We consider this
the honest, defensible framing of what differential testing here actually
proves.

## Findings: three verified behavior corrections

Differential testing surfaced three real discrepancies between our initial
assumptions and the original library's actual behavior. Each was resolved
by reading the original Python source directly, not by guessing:

### 1. Hamming distance on unequal-length strings

**Assumed:** Hamming distance requires equal-length inputs (the textbook
definition) and should error otherwise.

**Verified from source** (`textdistance/algorithms/edit_based.py`): the
original defaults to `truncate=False`, using Python's `zip_longest`
rather than `zip`. This means unequal-length strings are *not* rejected —
every position beyond the shorter string's length counts as an additional
mismatch.

**Fix:** rewrote `hamming()` to compute `mismatches_in_overlap +
abs(len(a) - len(b))` instead of erroring on length mismatch.

### 2. Needleman-Wunsch scoring weights

**Assumed:** standard textbook default weights — match: +1, mismatch: -1,
gap: -1.

**Verified from source:** the original library's default mismatch penalty
is **0**, not -1.

**Fix:** changed `NeedlemanWunsch::default()`'s `mismatch_penalty` from
`-1` to `0`. Also corrected our own unit test's expected value for the
classic `GATTACA`/`GCATGCU` textbook example — our original test assumed a
result of `0`, but the original library (with its actual default weights)
returns `3`.

### 3. Jaro-Winkler prefix boost threshold

**Assumed:** the Winkler prefix bonus is always applied on top of the Jaro
similarity score.

**Verified from source:** the original only applies the prefix boost when
the underlying Jaro similarity exceeds **0.7** — below that threshold, the
result is plain Jaro similarity with no boost.

**Fix:** added the `if jaro_similarity > 0.7 { jaro + boost } else { jaro }`
gate, matching the original's conditional logic exactly.

## Why this matters

All three corrections were only findable by actually running differential
tests against real reference data and reading the original source when a
mismatch appeared — none of them would have been caught by unit tests
written from memory or general algorithm knowledge alone. This is the
practical value of the "each ported function must pass the original tests"
requirement: it surfaces exactly this kind of subtle, easy-to-miss
behavioral divergence.

## What I'd do with more time

- Expand `scripts/generate_reference.py`'s test pairs to a larger,
  randomized fixed-seed set for broader differential coverage
- Port additional algorithms (Smith-Waterman and Ratcliff-Obershelp are
  natural next picks, though both carry more implementation risk than
  what's currently included)
- Add `proptest`-based property tests (e.g. `distance(a, a) == 0` for
  applicable algorithms, symmetry where the original guarantees it)
- Expand the qgram/bigram implementation to configurable `q` values with
  differential coverage beyond `q=2`
