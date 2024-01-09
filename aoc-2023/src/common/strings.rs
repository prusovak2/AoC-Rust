pub fn matches_substr_on_index(heap: &str, needle: &str, index: usize) -> bool {
    heap[index..].starts_with(needle)
}

pub fn get_char_at_index(vector: &Vec<String>, i: usize, j: usize) -> Option<char> {
    vector.get(i).and_then(|s| s.chars().nth(j))
}