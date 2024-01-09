pub fn matches_substr_on_index(heap: &str, needle: &str, index: usize) -> bool {
    heap[index..].starts_with(needle)
}