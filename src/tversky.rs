use std::collections::HashMap;

pub struct Tversky {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Tversky {
    fn default() -> Self {
        // Verified against real library: default reduces Tversky to Jaccard.
        Tversky { alpha: 1.0, beta: 1.0 }
    }
}

impl Tversky {
    pub fn similarity(&self, a: &str, b: &str) -> f64 {
        let mut counts_a = HashMap::new();
        for c in a.chars() {
            *counts_a.entry(c).or_insert(0) += 1;
        }
        let mut counts_b = HashMap::new();
        for c in b.chars() {
            *counts_b.entry(c).or_insert(0) += 1;
        }

        if counts_a.is_empty() && counts_b.is_empty() {
            return 1.0;
        }

        let mut keys: std::collections::HashSet<char> = counts_a.keys().copied().collect();
        keys.extend(counts_b.keys());

        let mut intersection = 0i32;
        let mut only_a = 0i32;
        let mut only_b = 0i32;

        for k in keys {
            let ca = *counts_a.get(&k).unwrap_or(&0);
            let cb = *counts_b.get(&k).unwrap_or(&0);
            intersection += ca.min(cb);
            only_a += (ca - ca.min(cb)).max(0);
            only_b += (cb - ca.min(cb)).max(0);
        }

        let denom = intersection as f64 + self.alpha * only_a as f64 + self.beta * only_b as f64;
        if denom == 0.0 {
            return 1.0;
        }
        intersection as f64 / denom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        let t = Tversky::default();
        assert_eq!(t.similarity("test", "test"), 1.0);
    }

    #[test]
    fn default_matches_jaccard() {
        let t = Tversky::default();
        let sim = t.similarity("GATTACA", "GCATGCU");
        assert!((sim - 0.4).abs() < 1e-9);
    }
}