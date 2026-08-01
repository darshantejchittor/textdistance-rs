/// Returns the length of the Longest Common Substring between two strings.
///
/// Unlike Longest Common Subsequence (LCS), a substring must be contiguous.
pub fn longest_common_substring_length(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());

    if n == 0 || m == 0 {
        return 0;
    }

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    let mut max_len = 0;

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                max_len = max_len.max(dp[i][j]);
            } else {
                // Substrings must be contiguous, so reset.
                dp[i][j] = 0;
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(longest_common_substring_length("kitten", "kitten"), 6);
    }

    #[test]
    fn contiguous_only() {
        // LCS would be "ace" (length 3), but the longest common
        // contiguous substring is only one character.
        assert_eq!(longest_common_substring_length("abcde", "ace"), 1);
    }

    #[test]
    fn real_substring_match() {
        // Common substring: "abc"
        assert_eq!(longest_common_substring_length("abcdef", "zabcx"), 3);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(longest_common_substring_length("", ""), 0);
        assert_eq!(longest_common_substring_length("abc", ""), 0);
    }

    #[test]
    fn no_common_substring() {
        assert_eq!(longest_common_substring_length("abc", "xyz"), 0);
    }

   #[test]
    fn overlapping_substring() {
        // Common substring: "issip"
        assert_eq!(longest_common_substring_length("mississippi", "issip"), 5);
    }
}