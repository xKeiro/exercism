use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = get_sorted_character_vector(word);
    possible_anagrams
        .iter()
        .filter(|&w| {
            let sorted_w = get_sorted_character_vector(w);
            *w.to_lowercase() != word.to_lowercase() && sorted_w == sorted_word
        })
        .cloned()
        .collect()
}

fn get_sorted_character_vector(string: &str) -> Vec<char> {
    let mut characters: Vec<char> = string.to_lowercase().chars().collect();
    characters.sort_unstable();
    characters
}
