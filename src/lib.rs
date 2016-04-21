use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub trait ShannonEntropy {
    fn entropy(&self) -> f32;
}

impl<T: AsRef<str>> ShannonEntropy for T {
    fn entropy(&self) -> f32 {
        shannon_entropy(self.as_ref())
    }
} 

/// Calculates the shannon entropy of 's'.
/// https://en.wiktionary.org/wiki/Shannon_entropy
///
/// # Arguments
///
/// * `s` - The string slice to calculate entropy for
///
/// # Examples
/// It's possible to call the function directly, as in the following example:
/// ```
/// let entropy: f32 = shannon_entropy::shannon_entropy("Hi there!");
/// ```
///
/// Alternatively, it is possible to import the `ShannonEntropy` trait and call
/// the function as if it were a method:
/// ```
/// use shannon_entropy::ShannonEntropy;
/// let entropy = "Hi, there!".entropy();
/// ```
pub fn shannon_entropy(s: &str) -> f32 {
    if s.is_empty() {
        return 0.0
    }
    
    let mut char_map: BTreeMap<char, usize> = BTreeMap::new();
    let mut ascii_map: [usize; 128] = [0; 128];
    let mut s_len = 0;

    for ch in s.chars() {
        s_len += 1;
        if ch.is_ascii() {
            ascii_map[ch as usize] += 1;
        } else {
            *char_map.entry(ch).or_insert(0) += 1;
        }
    }

    let s_len = (s_len as f32).round();
    let log_div = (2.0 as f32).ln();

    let result = char_map.values().chain(ascii_map.into_iter())
        .fold(0.0, |acc, &c| match c {
            0 => acc,
            c => acc + (c as f32 * (c as f32 / s_len).ln())
        }).abs();

    result / (s_len * log_div)
}


#[cfg(test)]
mod tests {
    use super::shannon_entropy;

    #[test]
    fn test_shannon() {
        let test_strings = vec![
            // Make sure we're consistent
            ("horse staple battery", shannon_entropy("horse staple battery")),
            // All-ASCII strings hit the fast path
            ("hello world", 2.845351),
            ("hello worldd", 2.8553884),
            ("a", 0.0),
            ("", 0.0),
            // Test non-ascii characters for slow path
            ("i ❤ rust", 2.7499998),
            ("ßℝ💣", 1.5849625),
            ("abc", 1.5849625),
            ("hello world💣", 3.0220554),
        ];
        
        for (test, answer) in test_strings {
            let entropy = shannon_entropy(test);
            assert_eq!(entropy, answer);
        }
    }
}