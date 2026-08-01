use std::collections::HashSet;

pub fn cosine_similarity(a: &str, b: &str) -> f64 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }
    if set_a.is_empty() || set_b.is_empty() {
        return 0.0;
    }

    let intersection = set_a.intersection(&set_b).count() as f64;
    let magnitude_a = (set_a.len() as f64).sqrt();
    let magnitude_b = (set_b.len() as f64).sqrt();

    intersection / (magnitude_a * magnitude_b)
}

pub fn cosine_distance(a: &str, b: &str) -> f64 {
    1.0 - cosine_similarity(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

   
    #[test]
    fn identical_strings() {
        let sim = cosine_similarity("test", "test");
        assert!((sim - 1.0).abs() < 1e-9);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(cosine_similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn both_empty() {
        assert_eq!(cosine_similarity("", ""), 1.0);
    }

    #[test]
    fn one_empty() {
        assert_eq!(cosine_similarity("abc", ""), 0.0);
    }

   #[test]
    fn distance_identical() {
        let dist = cosine_distance("test", "test");
        assert!(dist.abs() < 1e-9);
    }

    #[test]
    fn partial_overlap() {
        // {a,b,c} vs {b,c,d}: intersection=2, |A|=3, |B|=3
        // sim = 2 / (sqrt(3)*sqrt(3)) = 2/3
        let sim = cosine_similarity("abc", "bcd");
        assert!((sim - (2.0 / 3.0)).abs() < 1e-9);
    }
}