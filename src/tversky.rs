use std::collections::HashSet;

pub struct Tversky {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Tversky {
    fn default() -> Self {
        Tversky { alpha: 0.5, beta: 0.5 }
    }
}

impl Tversky {
    pub fn similarity(&self, a: &str, b: &str) -> f64 {
        let set_a: HashSet<char> = a.chars().collect();
        let set_b: HashSet<char> = b.chars().collect();

        if set_a.is_empty() && set_b.is_empty() {
            return 1.0;
        }

        let intersection = set_a.intersection(&set_b).count() as f64;
        let only_a = (set_a.len() as f64) - intersection;
        let only_b = (set_b.len() as f64) - intersection;

        intersection / (intersection + self.alpha * only_a + self.beta * only_b)
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
    fn default_matches_sorensen_dice() {
        let t = Tversky::default();
        let sim = t.similarity("night", "nacht");
        assert!((sim - 0.6).abs() < 1e-9);
    }

    #[test]
    fn both_empty() {
        let t = Tversky::default();
        assert_eq!(t.similarity("", ""), 1.0);
    }
}