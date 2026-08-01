pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    // Full 2D matrix, same shape as your existing Levenshtein.
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            dp[i][j] = std::cmp::min(
                std::cmp::min(
                    dp[i - 1][j] + 1,     // deletion
                    dp[i][j - 1] + 1,     // insertion
                ),
                dp[i - 1][j - 1] + cost,  // substitution
            );

            // Transposition: only possible if we're at least 2 chars into
            // both strings, and the current+previous pair is swapped.
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                dp[i][j] = dp[i][j].min(dp[i - 2][j - 2] + 1);
            }
        }
    }

    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(damerau_levenshtein("kitten", "kitten"), 0);
    }

    #[test]
    fn same_as_levenshtein_when_no_transposition() {
        assert_eq!(damerau_levenshtein("kitten", "sitting"), 3);
    }

    #[test]
    fn transposition_counts_as_one_edit() {
        // "ab" -> "ba" is 1 transposition, but 2 substitutions in plain Levenshtein
        assert_eq!(damerau_levenshtein("ab", "ba"), 1);
    }
    
    #[test]
    fn classic_transposition_example() {
        // OSA Damerau-Levenshtein returns 3 here.
        assert_eq!(damerau_levenshtein("CA", "ABC"), 3);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(damerau_levenshtein("", ""), 0);
        assert_eq!(damerau_levenshtein("abc", ""), 3);
    }
}