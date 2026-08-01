/// Length of the Longest Common Subsequence between two strings.
/// Unlike Levenshtein, this only counts characters kept in the same
/// relative order -- it doesn't allow substitution, just skipping.
pub fn lcs_length(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());

    if n == 0 || m == 0 {
        return 0;
    }

    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[n][m]
}

/// LCS-based distance: how many characters (combined, from both strings)
/// are NOT part of the longest common subsequence.
pub fn lcs_distance(a: &str, b: &str) -> usize {
    let la = a.chars().count();
    let lb = b.chars().count();
    let common = lcs_length(a, b);
    (la - common) + (lb - common)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(lcs_length("kitten", "kitten"), 6);
        assert_eq!(lcs_distance("kitten", "kitten"), 0);
    }

    #[test]
    fn classic_example() {
        // LCS of "ABCBDAB" and "BDCABA" is "BCBA" or "BDAB", length 4
        assert_eq!(lcs_length("ABCBDAB", "BDCABA"), 4);
    }

    #[test]
    fn no_common_subsequence() {
        assert_eq!(lcs_length("abc", "xyz"), 0);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(lcs_length("", ""), 0);
        assert_eq!(lcs_length("abc", ""), 0);
    }

    #[test]
    fn subsequence_not_substring() {
        // "ace" is a subsequence of "abcde" but not a contiguous substring
        assert_eq!(lcs_length("abcde", "ace"), 3);
    }

    #[test]
    fn distance_partial_overlap() {
        // lcs("abc", "bcd") = "bc", length 2
        // distance = (3-2) + (3-2) = 2
        assert_eq!(lcs_distance("abc", "bcd"), 2);
    }
}