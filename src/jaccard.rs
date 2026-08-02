use std::collections::{HashMap, HashSet};

fn char_counts(s: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

#[allow(dead_code)]
pub fn jaccard_distance(a: &str, b: &str) -> f64 {
    1.0 - jaccard_similarity(a, b)
}

pub fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let counts_a = char_counts(a);
    let counts_b = char_counts(b);

    if counts_a.is_empty() && counts_b.is_empty() {
        return 1.0;
    }

    let mut intersection = 0i32;
    let mut union = 0i32;

    let mut keys: HashSet<char> = counts_a.keys().copied().collect();
    keys.extend(counts_b.keys().copied());

    for k in keys {
        let ca = *counts_a.get(&k).unwrap_or(&0);
        let cb = *counts_b.get(&k).unwrap_or(&0);

        intersection += ca.min(cb);
        union += ca.max(cb);
    }

    if union == 0 {
        return 1.0;
    }

    intersection as f64 / union as f64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(jaccard_similarity("test", "test"), 1.0);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(jaccard_similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn both_empty() {
        assert_eq!(jaccard_similarity("", ""), 1.0);
    }

    #[test]
    fn multiset_example() {
        let sim = jaccard_similarity("ABCBDAB", "BDCABA");
        assert!((sim - 0.8571428571428571).abs() < 1e-9);
    }
}