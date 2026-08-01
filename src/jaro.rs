pub fn jaro_similarity(a: &str, b: &str) -> f64 {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (la, lb) = (a.len(), b.len());

    if la == 0 && lb == 0 {
        return 1.0;
    }
    if la == 0 || lb == 0 {
        return 0.0;
    }

    let match_distance = (la.max(lb) / 2).saturating_sub(1);

    let mut a_matches = vec![false; la];
    let mut b_matches = vec![false; lb];
    let mut matches = 0usize;

    for i in 0..la {
        let start = i.saturating_sub(match_distance);
        let end = (i + match_distance + 1).min(lb);

        for j in start..end {
            if b_matches[j] || a[i] != b[j] {
                continue;
            }

            a_matches[i] = true;
            b_matches[j] = true;
            matches += 1;
            break;
        }
    }

    if matches == 0 {
        return 0.0;
    }

    let mut transpositions = 0usize;
    let mut k = 0usize;

    for i in 0..la {
        if !a_matches[i] {
            continue;
        }

        while !b_matches[k] {
            k += 1;
        }

        if a[i] != b[k] {
            transpositions += 1;
        }

        k += 1;
    }

    let transpositions = transpositions / 2;

    let m = matches as f64;

    (m / la as f64
        + m / lb as f64
        + (m - transpositions as f64) / m)
        / 3.0
}

pub fn jaro_distance(a: &str, b: &str) -> f64 {
    1.0 - jaro_similarity(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(jaro_similarity("test", "test"), 1.0);
    }

    #[test]
    fn classic_martha_marhta() {
        let sim = jaro_similarity("MARTHA", "MARHTA");
        assert!((sim - 0.9444444444444445).abs() < 1e-9);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(jaro_similarity("", ""), 1.0);
        assert_eq!(jaro_similarity("a", ""), 0.0);
    }

    #[test]
    fn no_similarity() {
        assert_eq!(jaro_similarity("abc", "xyz"), 0.0);
    }

    #[test]
    fn distance_identical() {
        assert_eq!(jaro_distance("test", "test"), 0.0);
    }

    #[test]
    fn distance_no_similarity() {
        assert_eq!(jaro_distance("abc", "xyz"), 1.0);
    }
}