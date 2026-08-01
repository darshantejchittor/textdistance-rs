use std::collections::HashSet;

pub fn sorensen_dice_similarity(a: &str, b: &str) -> f64 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }

    let intersection = set_a.intersection(&set_b).count();

    (2.0 * intersection as f64) / (set_a.len() + set_b.len()) as f64
}

pub fn sorensen_dice_distance(a: &str, b: &str) -> f64 {
    1.0 - sorensen_dice_similarity(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(sorensen_dice_similarity("night", "night"), 1.0);
    }

    #[test]
    fn classic_example() {
        let sim = sorensen_dice_similarity("night", "nacht");
        assert!((sim - 0.6).abs() < 1e-9);
    }

    #[test]
    fn both_empty() {
        assert_eq!(sorensen_dice_similarity("", ""), 1.0);
    }

    #[test]
    fn no_overlap() {
        assert_eq!(sorensen_dice_similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn distance_identical() {
        assert_eq!(sorensen_dice_distance("night", "night"), 0.0);
    }

    #[test]
    fn distance_no_overlap() {
        assert_eq!(sorensen_dice_distance("abc", "xyz"), 1.0);
    }
}