/// Needleman-Wunsch global alignment score.
///
/// Unlike Levenshtein (which counts *edits*), this produces an alignment
/// *score* using configurable rewards/penalties: a match reward, a
/// mismatch penalty, and a gap penalty. Classic use case: DNA/protein
/// sequence alignment in bioinformatics.
pub struct NeedlemanWunsch {
    pub match_score: i32,
    pub mismatch_penalty: i32,
    pub gap_penalty: i32,
}

impl Default for NeedlemanWunsch {
    fn default() -> Self {
        // Common textbook defaults: +1 match, -1 mismatch, -1 gap.
        NeedlemanWunsch {
            match_score: 1,
            mismatch_penalty: -1,
            gap_penalty: -1,
        }
    }
}

impl NeedlemanWunsch {
    pub fn align_score(&self, a: &str, b: &str) -> i32 {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let (n, m) = (a.len(), b.len());

        let mut dp = vec![vec![0i32; m + 1]; n + 1];

        // First row/column: score of aligning against all gaps.
        for i in 0..=n {
            dp[i][0] = i as i32 * self.gap_penalty;
        }
        for j in 0..=m {
            dp[0][j] = j as i32 * self.gap_penalty;
        }

        for i in 1..=n {
            for j in 1..=m {
                let diag_score = if a[i - 1] == b[j - 1] {
                    self.match_score
                } else {
                    self.mismatch_penalty
                };

                let diagonal = dp[i - 1][j - 1] + diag_score;
                let up = dp[i - 1][j] + self.gap_penalty;
                let left = dp[i][j - 1] + self.gap_penalty;

                dp[i][j] = diagonal.max(up).max(left);
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings_score_max() {
        let nw = NeedlemanWunsch::default();
        // 4 matches, no mismatches/gaps -> score = 4
        assert_eq!(nw.align_score("test", "test"), 4);
    }

    #[test]
    fn empty_strings() {
        let nw = NeedlemanWunsch::default();
        assert_eq!(nw.align_score("", ""), 0);
    }

    #[test]
    fn one_empty_is_all_gaps() {
        let nw = NeedlemanWunsch::default();
        // "abc" vs "": 3 gap penalties of -1 each = -3
        assert_eq!(nw.align_score("abc", ""), -3);
    }

    #[test]
    fn classic_example() {
        let nw = NeedlemanWunsch::default();
        // GATTACA vs GCATGCU is a classic bioinformatics textbook example
        assert_eq!(nw.align_score("GATTACA", "GCATGCU"), 0);
    }

    #[test]
    fn custom_scoring_weights() {
        let nw = NeedlemanWunsch {
            match_score: 2,
            mismatch_penalty: -1,
            gap_penalty: -2,
        };
        // 4 matches * 2 = 8
        assert_eq!(nw.align_score("test", "test"), 8);
    }
}