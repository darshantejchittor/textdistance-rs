pub fn levenshtein(a: &str, b: &str) -> usize {

    if a == b {
        return 0;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let m = a_chars.len();
    let n = b_chars.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };

            dp[i][j] = std::cmp::min(
                std::cmp::min(
                    dp[i - 1][j] + 1,
                    dp[i][j - 1] + 1,
                ),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert_eq!(levenshtein("kitten", "kitten"), 0);
    }

    #[test]
    fn classic_example() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abc", ""), 3);
    }

    #[test]
    fn insertion() {
        assert_eq!(levenshtein("cat", "cats"), 1);
    }

    #[test]
    fn deletion() {
        assert_eq!(levenshtein("cats", "cat"), 1);
    }

    #[test]
    fn unicode() {
        assert_eq!(levenshtein("café", "cafe"), 1);
    }
}