use heapsize::HeapSizeOf;
/**
* Second attempt at implementing trie in Rust: use optional properly
* Now with additional heap size measurements
**/
use std::collections::hash_map::HashMap;

pub struct Trie {
    links: HashMap<char, Trie>,
    is_end: bool,
}

/** Figure out heap size of data structure */
impl HeapSizeOf for Trie {
    fn heap_size_of_children(&self) -> usize {
        self.links.heap_size_of_children()
    }
}

impl Trie {
    /** Initialize data structure. */
    pub fn new() -> Self {
        Trie {
            links: HashMap::new(),
            is_end: false,
        }
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.links.entry(c).or_insert(Trie::new());
        }
        node.is_end = true
    }

    /** Returns whether word is in the trie. */
    pub fn search(&mut self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.links.get_mut(&c) {
                None => return false,
                Some(n) => node = n,
            }
        }
        node.is_end
    }

    /** Returns whether any word in the trie starts with the given prefix. */
    pub fn starts_with(&mut self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.links.get_mut(&c) {
                None => return false,
                Some(n) => node = n,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_works() {
        let mut obj = Trie::new();
        assert_eq!(obj.links.len(), 0);
        obj.insert("test");
        assert_eq!(obj.links.len(), 1);
    }

    #[test]
    fn search_works() {
        let mut obj = Trie::new();
        obj.insert("test");
        assert_eq!(obj.search("test"), true);
        assert_ne!(obj.search("tes"), true);
    }

    #[test]
    fn starts_with_works() {
        let mut obj = Trie::new();
        obj.insert("test");
        assert_eq!(obj.starts_with("te"), true);
        assert_ne!(obj.starts_with("to"), true);
    }
}
