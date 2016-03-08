#![feature(test)]
use std::collections::BTreeMap;

pub fn shannon_entropy(s: &str) -> f32 {
    let mut char_map : BTreeMap<char, usize> = BTreeMap::new();
    for ch in s.chars() {
        *char_map.entry(ch).or_insert(0) += 1;
    }
    let s_len = (s.len() as f32).round();

    let mut result = 0.0;
    let log_div = (2.0 as f32).ln();

    for value in char_map.values() {
        let frequency: f32 = *value as f32 / s_len;
        result -= frequency * (frequency.ln() / log_div);
    }
    return result.abs()
}

#[cfg(test)]
mod tests {
    extern crate test;
    extern crate rand;

    use super::*;
    use self::test::Bencher;
    use self::rand::{thread_rng, Rng};

    #[test]
    fn test_shannon() {

        let test_strings = vec![
                                ("hello world", 2.8453512),
                                ("hello worldd", 2.8553884),
                                ("a", 0.0),
                                ("", 0.0),
                                ];
        for (test, answer) in test_strings {
            let entropy = shannon_entropy(test);
            assert_eq!(entropy, answer);
        }

    }


    #[bench]
    fn bench_shannon_empty(b: &mut Bencher) {
        b.iter(|| shannon_entropy(""));
    }

    #[bench]
    fn bench_shannon_small(b: &mut Bencher) {
        let s: String = thread_rng().gen_ascii_chars().take(64).collect();

        b.iter(|| shannon_entropy(&s));
    }

    #[bench]
    fn bench_shannon_medium(b: &mut Bencher) {
        let s: String = thread_rng().gen_ascii_chars().take(1024).collect();

        b.iter(|| shannon_entropy(&s));
    }

    #[bench]
    fn bench_shannon_large(b: &mut Bencher) {
        let s: String = thread_rng().gen_ascii_chars().take(65536).collect();

        b.iter(|| shannon_entropy(&s));
    }
}
