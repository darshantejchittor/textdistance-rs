# Benchmarks

Timing comparison across all 15 ported algorithms, measured in release mode
(`cargo run --release`) on the same machine, using `std::time::Instant`.
Each figure is the total wall-clock time for **10,000 iterations** of that
algorithm on the given input pair (not per-call time).

Test pairs cover three shapes: short/similar-length words (classic edit
distance example), a short transposition-heavy pair, and two longer,
near-identical sentences.

## Results

### `"kitten"` vs `"sitting"`

| Algorithm | Time (10,000 iterations) |
|---|---|
| hamming | 14.93 ms |
| jaro | 16.22 ms |
| bag_distance | 23.84 ms |
| sorensen | 25.93 ms |
| cosine | 26.27 ms |
| jaro_winkler | 30.06 ms |
| lcs (subsequence) | 39.28 ms |
| longest_common_substring | 43.71 ms |
| jaccard | 52.23 ms |
| qgram (bigram) | 59.59 ms |
| levenshtein | 64.25 ms |
| damerau_levenshtein | 81.87 ms |
| tversky | 89.78 ms |
| needleman_wunsch | 21.15 ms |
| overlap | 167.73 ms |

### `"ABCBDAB"` vs `"BDCABA"`

| Algorithm | Time (10,000 iterations) |
|---|---|
| hamming | 5.59 ms |
| jaro | 8.86 ms |
| overlap | 9.67 ms |
| cosine | 11.38 ms |
| jaro_winkler | 15.28 ms |
| bag_distance | 6.94 ms |
| needleman_wunsch | 20.48 ms |
| tversky | 26.49 ms |
| levenshtein | 28.74 ms |
| lcs (subsequence) | 28.81 ms |
| longest_common_substring | 29.78 ms |
| jaccard | 32.82 ms |
| damerau_levenshtein | 38.65 ms |
| qgram (bigram) | 48.41 ms |
| sorensen | 50.77 ms |

### `"the quick brown fox"` vs `"the quick brown fax"`

| Algorithm | Time (10,000 iterations) |
|---|---|
| hamming | 5.99 ms |
| bag_distance | 38.19 ms |
| jaro | 38.45 ms |
| jaro_winkler | 46.87 ms |
| sorensen | 46.45 ms |
| cosine | 53.55 ms |
| longest_common_substring | 60.36 ms |
| jaccard | 58.69 ms |
| levenshtein | 63.15 ms |
| tversky | 72.73 ms |
| damerau_levenshtein | 74.74 ms |
| overlap | 81.88 ms |
| lcs (subsequence) | 88.98 ms |
| qgram (bigram) | 92.59 ms |
| needleman_wunsch | 103.70 ms |

## Observations

- **Hamming is consistently the fastest** across all three pairs — expected,
  since it's a single linear pass with no allocation-heavy data structures.
- **Multiset/set-based algorithms** (Jaccard, Sorensen-Dice, Cosine, Overlap,
  Tversky) show more variance between pairs than edit-based algorithms,
  since their cost depends on character-set/multiset construction
  (`HashMap`/`HashSet` allocation) rather than purely on string length.
- **Edit-based algorithms with full DP matrices** (Levenshtein,