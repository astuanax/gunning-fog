extern crate regex;

use regex::Regex;

pub fn gunning_fog_index(text: &str) -> f64 {
    let sentence_count = count_sentences(text);
    let word_count = count_words(text);
    let complex_word_count = count_complex_words(text);

    if sentence_count == 0 || word_count == 0 {
        return 0.0;
    }

    let average_sentence_length = word_count as f64 / sentence_count as f64;
    let percentage_complex_words = complex_word_count as f64 / word_count as f64 * 100.0;

    0.4 * (average_sentence_length + percentage_complex_words)
}

fn count_sentences(text: &str) -> usize {
    let re = Regex::new(r"[.!?]").unwrap();
    re.find_iter(text).count()
}

fn count_words(text: &str) -> usize {
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text).count()
}

fn count_complex_words(text: &str) -> usize {
    let re = Regex::new(r"\b\w{3,}\b").unwrap();
    re.find_iter(text).filter(|word| syllable_count(word.as_str()) >= 3).count()
}

fn syllable_count(word: &str) -> usize {
    let vowels = Regex::new(r"[aeiouy]+").unwrap();
    let mut count = vowels.find_iter(word).count();

    if word.ends_with('e') {
        count -= 1;
    }

    count.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gunning_fog_index() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let index = gunning_fog_index(text);
        assert!(index == 3.6);
    }

    #[test]
    fn test_syllable_count() {
        assert_eq!(syllable_count("complex"), 2);
        assert_eq!(syllable_count("words"), 1);
        assert_eq!(syllable_count("syllable"), 2);
        assert_eq!(syllable_count("test"), 1);
    }
}
