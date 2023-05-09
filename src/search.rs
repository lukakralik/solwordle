use crate::utils::*;
use regex::Regex;

pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let s_len = s.chars().count();
    let t_len = t.chars().count();

    if s_len == 0 {
        return t_len;
    }

    if t_len == 0 {
        return s_len;
    }

    let mut distance_matrix = vec![vec![0; t_len + 1]; s_len + 1];
    // 2D array of costs

    for i in 0..=s_len {
        distance_matrix[i][0] = i;
    }

    for j in 0..=t_len {
        distance_matrix[0][j] = j;
    }

    for (i, s_char) in s.chars().enumerate() {
        for (j, t_char) in t.chars().enumerate() {
            let cost = if s_char == t_char { 0 } else { 1 };

            distance_matrix[i + 1][j + 1] = std::cmp::min(
                distance_matrix[i][j + 1] + 1,
                std::cmp::min(distance_matrix[i + 1][j] + 1, distance_matrix[i][j] + cost),
            );
        }
    }

    distance_matrix[s_len][t_len]
}

pub fn suitable_sequences(green: String, yellow: Vec<String>, gray: Vec<String>, words: Vec<String>) -> Vec<String> {
    // green = A*O*US
    // yellow = A, O, U, S, M, G
    // gray = L, D, C, B, N..
    let mut suiting = words.clone();
    for word in words.iter() {
        for (i, c) in green.chars().enumerate() {
            if word.as_bytes()[i] != '*' as u8 {
                if c as u8 != green.as_bytes()[i] {
                    if let Some(index) = suiting.iter().position(|x| x == word) {
                        suiting.remove(index);
                    }
                    break
                }
            }
        }

        for pos in yellow.iter() {
            if !word.contains(pos) {
                if let Some(index) = suiting.iter().position(|x| x == word) {
                    suiting.remove(index);
                }
                break
            }
        }

        for pos in gray.iter() {
            if word.contains(pos) {
                if let Some(index) = suiting.iter().position(|x| x == word) {
                    suiting.remove(index);
                }
                break
            }
        }

        suiting.push(word.to_owned());
    }
    to_upper(suiting)
    // regex alternative: /\bG\wA*E\wT\b/
}

// alternative
// accepts guessed words in the form XX-X-X
pub fn filter_words(words: Vec<String>, guessed_word: &str, included_letters: &[char], excluded_letters: &[char]) -> Vec<String> {
    let mut filtered_words = Vec::new();

    // regex pattern
    let regex_str = guessed_word.replace("-", "[A-Z]");
    let regex = Regex::new(&regex_str).unwrap();

    for word in words {
        // this situation shouldn't be reached if the correct data was passed
        if word.len() != guessed_word.len() {
            continue;
        }
        // doesn't fit in the RegEx pattern
        if !regex.is_match(word.as_ref()) {
            continue;
        }
        // see if word contains the mandatory included characters
        let mut all_incl_letters = true;
        for &letter in included_letters {
            if !word.contains(letter) {
                all_incl_letters = false;
                break;
            }
        }
        if !all_incl_letters {
            continue;
        }
        // erase the word if it contains forbidden characters
        for &letter in excluded_letters {
            if word.contains(letter) {
                continue;
            }
        }
        filtered_words.push(word.to_owned());
    }

    filtered_words
}
