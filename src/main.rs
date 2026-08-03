mod levenshtein;
mod hamming;
mod jaccard;
mod sorensen;
mod jaro;
mod jaro_winkler;
mod cosine;
mod damerau;
mod lcs;
mod needleman_wunsch;
mod longest_common_substring;
mod overlap;
mod tversky;
mod bag_distance;
mod qgram;


fn main() {
    let distance = levenshtein::levenshtein("kitten", "sitting");

    println!(
        "Distance between 'kitten' and 'sitting' = {}",
        distance
    );

    run_benchmarks();
}

fn run_benchmarks() {
    use std::time::Instant;

    let pairs = [
        ("kitten", "sitting"),
        ("ABCBDAB", "BDCABA"),
        ("the quick brown fox", "the quick brown fax"),
    ];

    println!("\n--- Benchmark: 10,000 iterations per algorithm per pair ---\n");

    for (a, b) in pairs.iter() {
        println!("Pair: {:?} vs {:?}", a, b);

        macro_rules! bench {
            ($name:expr, $call:expr) => {
                let start = Instant::now();
                for _ in 0..10_000 {
                    $call;
                }
                println!("  {:<22} {:?}", $name, start.elapsed());
            };
        }

        bench!("levenshtein:", levenshtein::levenshtein(a, b));
        bench!("hamming:", hamming::hamming(a, b));
        bench!("damerau_levenshtein:", damerau::damerau_levenshtein(a, b));
        bench!("jaccard:", jaccard::jaccard_similarity(a, b));
        bench!("sorensen:", sorensen::sorensen_dice_similarity(a, b));
        bench!("cosine:", cosine::cosine_similarity(a, b));
        bench!("overlap:", overlap::overlap_similarity(a, b));
        bench!("tversky:", tversky::Tversky::default().similarity(a, b));
        bench!("bag_distance:", bag_distance::bag_distance(a, b));
        bench!("jaro:", jaro::jaro_similarity(a, b));
        bench!("jaro_winkler:", jaro_winkler::jaro_winkler_similarity(a, b));
        bench!("lcs (subsequence):", lcs::lcs_length(a, b));
        bench!("longest_common_substr:", longest_common_substring::longest_common_substring_length(a, b));
        bench!(
            "needleman_wunsch:",
            needleman_wunsch::NeedlemanWunsch::default().align_score(a, b)
        );
        bench!("qgram (bigram):", qgram::qgram_distance(a, b, 2));

        println!();
    }
}

#[cfg(test)]
mod differential_tests {
    use super::*;
    use serde_json::Value;
    use std::fs;


    #[test]
    fn matches_python_reference() {
        let data = fs::read_to_string("tests/fixtures/reference_values.json")
            .expect("run: python scripts/generate_reference.py first");
        let cases: Vec<Value> = serde_json::from_str(&data).expect("valid json");

        let mut failures = Vec::new();

        for case in &cases {
            let a = case["a"].as_str().unwrap();
            let b = case["b"].as_str().unwrap();

            check_usize(&mut failures, "levenshtein", a, b, levenshtein::levenshtein(a, b), &case["levenshtein"]);
            check_usize(&mut failures, "hamming", a, b, hamming::hamming(a, b), &case["hamming"]);
            check_f64(&mut failures, "jaccard", a, b, jaccard::jaccard_similarity(a, b), &case["jaccard"]);
            check_f64(&mut failures, "sorensen", a, b, sorensen::sorensen_dice_similarity(a, b), &case["sorensen"]);
            check_f64(&mut failures, "jaro", a, b, jaro::jaro_similarity(a, b), &case["jaro"]);
            check_f64(&mut failures, "jaro_winkler", a, b, jaro_winkler::jaro_winkler_similarity(a, b), &case["jaro_winkler"]);
            check_f64(&mut failures, "cosine", a, b, cosine::cosine_similarity(a, b), &case["cosine"]);
            check_usize(&mut failures, "damerau_levenshtein", a, b, damerau::damerau_levenshtein(a, b), &case["damerau_levenshtein"]);
            check_usize(&mut failures, "lcsseq", a, b, lcs::lcs_length(a, b), &case["lcsseq"]);

            let nw = needleman_wunsch::NeedlemanWunsch::default();
            check_i32(&mut failures, "needleman_wunsch", a, b, nw.align_score(a, b), &case["needleman_wunsch"]);

            check_usize(&mut failures, "lcsstr", a, b, longest_common_substring::longest_common_substring_length(a, b), &case["lcsstr"]);
            check_f64(&mut failures, "overlap", a, b, overlap::overlap_similarity(a, b), &case["overlap"]);

            let tv = tversky::Tversky::default();
            check_f64(&mut failures, "tversky", a, b, tv.similarity(a, b), &case["tversky"]);

            check_usize(&mut failures, "bag", a, b, bag_distance::bag_distance(a, b), &case["bag"]);
            check_usize(&mut failures, "qgram_bigram", a, b, qgram::qgram_distance(a, b, 2), &case["qgram_bigram"]);
        }

        if !failures.is_empty() {
            println!("{} mismatch(es):", failures.len());

            for failure in &failures {
                println!("{failure}");
            }

            panic!("Python reference mismatch");
        }
        }


    fn check_usize(failures: &mut Vec<String>, name: &str, a: &str, b: &str, got: usize, expected: &Value) {
        let want = expected.as_i64().unwrap() as usize;
        if got != want {
            failures.push(format!("{name}({a:?}, {b:?}): got {got}, want {want}"));
        }
    }

    fn check_i32(failures: &mut Vec<String>, name: &str, a: &str, b: &str, got: i32, expected: &Value) {
        let want = expected.as_f64().unwrap() as i32;
        if got != want {
            failures.push(format!("{name}({a:?}, {b:?}): got {got}, want {want}"));
        }
    }

    fn check_f64(failures: &mut Vec<String>, name: &str, a: &str, b: &str, got: f64, expected: &Value) {
        let want = expected.as_f64().unwrap();
       if (got - want).abs() > 1e-6 {
            failures.push(format!("{name}({a:?}, {b:?}): got {got}, want {want}"));
        }
    }
}