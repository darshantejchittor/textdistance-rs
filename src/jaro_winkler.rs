use crate::jaro::jaro_similarity;

pub fn jaro_winkler_similarity(a: &str, b: &str) -> f64 {
    let jaro_sim = jaro_similarity(a, b);

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let max_prefix = 4.min(a_chars.len()).min(b_chars.len());

    let mut prefix_len = 0;
    for i in 0..max_prefix {
        if a_chars[i] == b_chars[i] {
            prefix_len += 1;
        } else {
            break;
        }
    }

    const PREFIX_WEIGHT: f64 = 0.1;
    jaro_sim + (prefix_len as f64 * PREFIX_WEIGHT * (1.0 - jaro_sim))
}

pub fn jaro_winkler_distance(a: &str, b: &str) -> f64 {
    1.0 - jaro_winkler_similarity(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jaro::jaro_similarity;

    #[test]
    fn identical_strings() {
        assert_eq!(jaro_winkler_similarity("test", "test"), 1.0);
    }

    #[test]
    fn boosts_shared_prefix() {
        let jaro = jaro_similarity("DWAYNE", "DUANE");
        let jw = jaro_winkler_similarity("DWAYNE", "DUANE");
        assert!(jw >= jaro);
    }

    #[test]
    fn no_shared_prefix_equals_jaro() {
        let jaro = jaro_similarity("abc", "xyz");
        let jw = jaro_winkler_similarity("abc", "xyz");
        assert_eq!(jw, jaro);
    }

    #[test]
    fn classic_dwayne_duane() {
        let sim = jaro_winkler_similarity("DWAYNE", "DUANE");
        assert!((sim - 0.84).abs() < 0.01);
    }

    #[test]
    fn distance_identical() {
        assert_eq!(jaro_winkler_distance("test", "test"), 0.0);
    }

    #[test]
    fn distance_empty() {
        assert_eq!(jaro_winkler_distance("", ""), 0.0);
    }
}