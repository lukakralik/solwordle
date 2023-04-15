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

// Time complexity: O(m*n)
// Auxiliary complexity: O(m*n)
