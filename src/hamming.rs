pub fn hamming(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let max_len = a_chars.len().max(b_chars.len());

    let mut distance = 0;

    for i in 0..max_len {
        let left = a_chars.get(i);
        let right = b_chars.get(i);

        if left != right {
            distance += 1;
        }
    }

        distance
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical() {
        assert_eq!(hamming("abc", "abc"), 0);
    }

    #[test]
    fn classic() {
        assert_eq!(hamming("karolin", "kathrin"), 3);
    }


    #[test]
    fn different_lengths() {
        assert_eq!(hamming("abc", "ab"), 1);
        assert_eq!(hamming("abc", "abcdef"), 3);
    }



    #[test]
    fn unicode() {
        assert_eq!(hamming("café", "caff"), 1);
    }
}