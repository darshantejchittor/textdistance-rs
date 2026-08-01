use std::collections::HashMap;

pub fn bag_distance(a: &str, b: &str) -> usize {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in a.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in b.chars() {
        *counts.entry(c).or_insert(0) -= 1;
    }

    let positive: i32 = counts.values().filter(|&&v| v > 0).sum();
    let negative: i32 = counts.values().filter(|&&v| v < 0).map(|v| -v).sum();

    positive.max(negative) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(bag_distance("test", "test"), 0);
    }

    #[test]
    fn classic_example() {
        assert_eq!(bag_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(bag_distance("", ""), 0);
        assert_eq!(bag_distance("abc", ""), 3);
    }

    #[test]
    fn anagram_distance_zero() {
        assert_eq!(bag_distance("abc", "cab"), 0);
    }
}