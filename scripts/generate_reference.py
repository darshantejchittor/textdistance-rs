import json
import textdistance

test_pairs = [
    ("", ""),
    ("abc", ""),
    ("", "abc"),
    ("kitten", "kitten"),
    ("kitten", "sitting"),
    ("MARTHA", "MARHTA"),
    ("DWAYNE", "DUANE"),
    ("night", "nacht"),
    ("café", "cafe"),
    ("abc", "xyz"),
    ("abc", "bcd"),
    ("ab", "ba"),
    ("CA", "ABC"),
    ("karolin", "kathrin"),
    ("abc", "abcd"),
    ("test", "test"),
    ("ABCBDAB", "BDCABA"),
    ("abcde", "ace"),
    ("GATTACA", "GCATGCU"),
    ("abcdef", "zabcx"),
]

# Each entry: (result_key, callable). Wrapped individually so one bad
# name doesn't kill the whole run -- we'll see exactly which ones fail.
algorithms = {
    "levenshtein": lambda a, b: textdistance.levenshtein(a, b),
    "hamming": lambda a, b: textdistance.hamming(a, b),
    "jaccard": lambda a, b: textdistance.jaccard.normalized_similarity(a, b),
    "sorensen": lambda a, b: textdistance.sorensen.normalized_similarity(a, b),
    "jaro": lambda a, b: textdistance.jaro.normalized_similarity(a, b),
    "jaro_winkler": lambda a, b: textdistance.jaro_winkler.normalized_similarity(a, b),
    "cosine": lambda a, b: textdistance.cosine.normalized_similarity(a, b),
    "damerau_levenshtein": lambda a, b: textdistance.damerau_levenshtein(a, b),
    "lcsseq": lambda a, b: textdistance.lcsseq.similarity(a, b),
    "needleman_wunsch": lambda a, b: textdistance.needleman_wunsch(a, b),
    "lcsstr": lambda a, b: len(textdistance.lcsstr(a, b)),
    "overlap": lambda a, b: textdistance.overlap.normalized_similarity(a, b),
    "tversky": lambda a, b: textdistance.tversky.normalized_similarity(a, b),
    "bag": lambda a, b: textdistance.bag(a, b),
    "qgram_bigram": lambda a, b: sum(
        (
            __import__("collections").Counter(
                [a[i:i+2] for i in range(len(a)-1)] if len(a) >= 2 else []
            )
            - __import__("collections").Counter(
                [b[i:i+2] for i in range(len(b)-1)] if len(b) >= 2 else []
            )
        ).values()
    ) + sum(
        (
            __import__("collections").Counter(
                [b[i:i+2] for i in range(len(b)-1)] if len(b) >= 2 else []
            )
            - __import__("collections").Counter(
                [a[i:i+2] for i in range(len(a)-1)] if len(a) >= 2 else []
            )
        ).values()
    ),
}

errors = {}
results = []

for a, b in test_pairs:
    entry = {"a": a, "b": b}
    for name, fn in algorithms.items():
        try:
            entry[name] = fn(a, b)
        except Exception as e:
            errors.setdefault(name, str(e))
            entry[name] = None
    results.append(entry)

with open("tests/fixtures/reference_values.json", "w") as f:
    json.dump(results, f, indent=2)

print(f"Generated {len(results)} test cases -> tests/fixtures/reference_values.json")

if errors:
    print("\n--- ALGORITHMS THAT FAILED (need fixing) ---")
    for name, err in errors.items():
        print(f"  {name}: {err}")
else:
    print("\nAll algorithms ran successfully.")