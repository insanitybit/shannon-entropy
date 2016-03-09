# shannon-entropy
A rust library to calculate the Shannon entropy of a string.

# Usage
Available on crates.io

Add this to your Cargo.toml

```toml
[dependencies]
shannon-entropy = "0.2"
```

# Example

```rust
fn main() {
  let test_strings = vec![
                          ("hello world", 2.8453512),
                          ("hello worldd", 2.8553884),
                          ("a", 0.0),
                          ("", 0.0),
                          ];

  for (test, answer) in test_strings {
      let entropy: f32 = shannon_entropy(test);
      assert_eq!(entropy, answer);
  }  
}
```
