use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&w| {
            w.len() == word.len() &&
            *w.to_lowercase() != word.to_lowercase() && 
            get_sorted_word(w) == get_sorted_word(word)
        })
        .cloned()
        .collect()
}

fn get_sorted_word(string: &str) -> Vec<char> {
    let mut characters: Vec<char> = string.to_lowercase().chars().collect();
    characters.sort_unstable();
    characters
}
