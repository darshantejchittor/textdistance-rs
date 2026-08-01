use std::collections::HashSet;

pub fn overlap_similarity(a: &str, b: &str) -> f64 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }
    if set_a.is_empty() || set_b.is_empty() {
        return 0.0;
    }

    let intersection = set_a.intersection(&set_b).count() as f64;
    intersection / set_a.len().min(set_b.len()) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(overlap_similarity("test", "test"), 1.0);
    }

    #[test]
    fn subset_is_full_overlap() {
        assert_eq!(overlap_similarity("ab", "abc"), 1.0);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(overlap_similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn both_empty() {
        assert_eq!(overlap_similarity("", ""), 1.0);
    }
}