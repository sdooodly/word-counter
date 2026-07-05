use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn count_words(text: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;

    for ch in text.chars() {
        let is_word_char = ch.is_alphanumeric();

        if is_word_char && !in_word {
            count += 1;
            in_word = true;
        } else if !is_word_char {
            in_word = false;
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap_or_else(|| "sample.txt".to_string());
    let path = Path::new(&file_path);

    if !path.exists() {
        eprintln!("File not found: {}", path.display());
        return Err(format!("File not found: {}", path.display()).into());
    }

    let contents = fs::read_to_string(path)?;
    let word_count = count_words(&contents);

    println!("Word count: {word_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::count_words;

    #[test]
    fn counts_words_across_whitespace() {
        assert_eq!(count_words("Hello Rust World"), 3);
        assert_eq!(count_words("  Hello   Rust\nWorld  "), 3);
    }

    #[test]
    fn counts_zero_for_empty_input() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   \n\t  "), 0);
    }

    #[test]
    fn counts_words_with_punctuation_and_unicode() {
        assert_eq!(count_words("Hello, world!"), 2);
        assert_eq!(count_words("こんにちは、世界"), 2);
        assert_eq!(count_words("one-two_three"), 3);
    }
}
