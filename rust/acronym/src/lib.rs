pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| word.chars().take(1).chain(extract_camelcase_chars(word)))
        .collect::<String>()
        .to_ascii_uppercase()
}

fn extract_camelcase_chars(word: &str) -> impl Iterator<Item = char> + '_ {
    word.chars()
        .skip_while(|c| c.is_uppercase())
        .filter(|c| c.is_uppercase())
}
