use std::cmp::min;

pub fn edit_distance(a: &str, b: &str, m: u32, n:u32) -> u32 {
    if m == 0 {
        return n;
    }

    if n == 0 {
        return m;
    }

    if a.chars().nth((m-1) as usize).unwrap() == b.chars().nth((n-1) as usize).unwrap() {
        return edit_distance(a, b, m-1, n-1);
    }

    return 1 + min(
               min(
                   edit_distance(a, b, m, n-1),
                   edit_distance(a, b, m-1, n)),
                   edit_distance(a, b, m-1, n-1));
}
