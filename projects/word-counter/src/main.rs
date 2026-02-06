use std::collections::HashMap;
use std::env;
use std::fs;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let cleaned: String = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect();

        if !cleaned.is_empty() {
            *counts.entry(cleaned).or_insert(0) += 1;
        }
    }

    counts
}

fn display_results(counts: &HashMap<String, usize>, top_n: usize) {
    let mut sorted: Vec<(&String, &usize)> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    let total: usize = counts.values().sum();
    let unique = counts.len();

    println!("\nTop {} words:", top_n.min(sorted.len()));
    for (rank, (word, count)) in sorted.iter().take(top_n).enumerate() {
        println!("  {:>2}. {:<15} — {}", rank + 1, word, count);
    }

    println!("\nTotal: {total} words, {unique} unique");
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: word-counter <file>".to_string());
    }

    let filename = &args[1];
    println!("Reading: {filename}");

    let text =
        fs::read_to_string(filename).map_err(|e| format!("Error reading '{filename}': {e}"))?;

    let counts = count_words(&text);

    if counts.is_empty() {
        println!("No words found in the file.");
        return Ok(());
    }

    display_results(&counts, 10);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_simple() {
        let counts = count_words("hello world hello");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_count_case_insensitive() {
        let counts = count_words("Hello HELLO hello");
        assert_eq!(counts.get("hello"), Some(&3));
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn test_count_punctuation() {
        let counts = count_words("hello, world! hello.");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_count_empty() {
        let counts = count_words("");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_whitespace_only() {
        let counts = count_words("   \n\t  ");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_preserves_apostrophes() {
        let counts = count_words("don't can't won't");
        assert_eq!(counts.get("don't"), Some(&1));
        assert_eq!(counts.get("can't"), Some(&1));
    }
}
