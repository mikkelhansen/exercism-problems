use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sorted_chars(&word_lower);

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| {
            if candidate.len() != word.len() {
                return false;
            }
            let candidate_lower = candidate.to_lowercase();
            candidate_lower != word_lower && sorted_chars(&candidate_lower) == word_sorted
        })
        .collect()
}

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars
}
