use std::collections::HashMap;

fn char_counts(s: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

pub fn overlap_similarity(a: &str, b: &str) -> f64 {
    let counts_a = char_counts(a);
    let counts_b = char_counts(b);
    let total_a: i32 = counts_a.values().sum();
    let total_b: i32 = counts_b.values().sum();

    if total_a == 0 && total_b == 0 {
        return 1.0;
    }
    if total_a == 0 || total_b == 0 {
        return 0.0;
    }

    let mut intersection = 0i32;
    for (k, &ca) in &counts_a {
        let cb = *counts_b.get(k).unwrap_or(&0);
        intersection += ca.min(cb);
    }

    intersection as f64 / total_a.min(total_b) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(overlap_similarity("test", "test"), 1.0);
    }

    #[test]
    fn multiset_example() {
        let sim = overlap_similarity("GATTACA", "GCATGCU");
        assert!((sim - 0.5714285714285714).abs() < 1e-9);
    }
}