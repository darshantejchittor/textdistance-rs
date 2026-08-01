use std::collections::HashMap;

/// Generates overlapping q-grams (n-grams) from a string.
///
/// Example:
/// ```text
/// qgrams("hello", 2) -> ["he", "el", "ll", "lo"]
/// ```
pub fn qgrams(s: &str, q: usize) -> Vec<String> {
    if q == 0 {
        return Vec::new();
    }

    let chars: Vec<char> = s.chars().collect();

    if chars.len() < q {
        return Vec::new();
    }

    (0..=chars.len() - q)
        .map(|i| chars[i..i + q].iter().collect())
        .collect()
}

/// Computes the Q-gram distance between two strings.
///
/// The distance is the sum of the absolute differences between the
/// frequencies of each q-gram in the two strings.
///
/// Smaller values indicate more similar strings.
pub fn qgram_distance(a: &str, b: &str, q: usize) -> usize {
    let grams_a = qgrams(a, q);
    let grams_b = qgrams(b, q);

    let mut counts: HashMap<String, i32> = HashMap::new();

    for gram in grams_a {
        *counts.entry(gram).or_default() += 1;
    }

    for gram in grams_b {
        *counts.entry(gram).or_default() -= 1;
    }

    counts
        .values()
        .map(|count| count.unsigned_abs() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(qgram_distance("test", "test", 2), 0);
    }

    #[test]
    fn bigrams_of_hello() {
        assert_eq!(
            qgrams("hello", 2),
            vec![
                "he".to_string(),
                "el".to_string(),
                "ll".to_string(),
                "lo".to_string()
            ]
        );
    }

    #[test]
    fn distinguishes_order_unlike_bag() {
        assert_eq!(qgram_distance("ab", "ba", 2), 2);
    }

    #[test]
    fn too_short_for_q_gives_empty() {
        assert_eq!(qgrams("a", 2), Vec::<String>::new());
    }

    #[test]
    fn zero_q_gives_empty() {
        assert_eq!(qgrams("hello", 0), Vec::<String>::new());
    }

    #[test]
    fn classic_example() {
        // "night" -> ni ig gh ht
        // "nacht" -> na ac ch ht
        // Shared: "ht"
        assert_eq!(qgram_distance("night", "nacht", 2), 6);
    }
}