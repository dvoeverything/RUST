use std::collections::HashMap;

/// Return a map: word → frequency, case-insensitive
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut count = HashMap::new();

    for raw in text.split_whitespace() {
        // strip punctuation (optional but handy)
        let cleaned = raw
            .trim_matches(|c: char| !c.is_alphanumeric())  // keeps letters/digits
            .to_lowercase();                               //  force lowercase

        if cleaned.is_empty() { continue }                // skip "" produced by all-punct tokens
        *count.entry(cleaned).or_insert(0) += 1;          //  insert / increment
    }

    count
}

fn main() {
    let text = "The quickest brown fox jumped over the lazy dog \
                because the dog was lazy and the fox was not";

    let word_count = count_words(text);
    println!("{word_count:#?}");
}