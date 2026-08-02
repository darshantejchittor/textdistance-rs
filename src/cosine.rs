use std::collections::HashMap;

fn char_counts(s: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

pub fn cosine_similarity(a: &str, b: &str) -> f64 {
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

    intersection as f64 / ((total_a as f64) * (total_b as f64)).sqrt()
}

#[allow(dead_code)]
pub fn cosine_distance(a: &str, b: &str) -> f64 {
    1.0 - cosine_similarity(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(cosine_similarity("test", "test"), 1.0);
    }

    #[test]
    fn multiset_example() {
        let sim = cosine_similarity("ABCBDAB", "BDCABA");
        assert!((sim - 0.9258200997725514).abs() < 1e-9);
    }
}