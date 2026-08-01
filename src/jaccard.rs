use std::collections::HashSet;

pub fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }

    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();

    intersection as f64 / union as f64
}

pub fn jaccard_distance(a: &str, b: &str) -> f64 {
    1.0 - jaccard_similarity(a, b)
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
    fn partial_overlap() {
        assert_eq!(jaccard_similarity("abc", "bcd"), 0.5);
    }

    #[test]
    fn both_empty() {
        assert_eq!(jaccard_similarity("", ""), 1.0);
    }
}