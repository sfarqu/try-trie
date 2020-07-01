/**
* First attempt at implementing trie in Rust
* Now with added heap size measurements
**/
use std::collections::hash_map::{HashMap};
use heapsize::HeapSizeOf;

pub struct Trie {
    links: HashMap<char, Trie>,
    is_end: bool,
    is_empty: bool // Whoops I sort of manually re-implemented the concept of optionals?
}

impl Default for Trie {
    fn default() -> Self { Trie { links: HashMap::new(), is_end: false, is_empty: true } }
}

/** Figure out heap size of data structure */
impl HeapSizeOf for Trie {
    fn heap_size_of_children(&self) -> usize {
        self.links.heap_size_of_children() +
            self.is_end.heap_size_of_children() +
            self.is_empty.heap_size_of_children()
    }
}

impl Trie {

    /** Initialize data structure. */
    pub fn new() -> Self {
        Trie { links: HashMap::new(), is_end: false, is_empty: false }
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.links.entry(c).or_insert(Trie::new());
            node.is_empty = false
        }
        node.is_end = true
    }

    /** Returns whether word is in the trie. */
    // would rather not pass self as mutable but using self as root node requires it
    pub fn search(&mut self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            node = node.links.entry(c).or_default();
            if node.is_empty {
                return false
            }
        }
        node.is_end
    }

    /** Returns whether any word in the trie starts with the given prefix. */
    pub fn starts_with(&mut self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            node = node.links.entry(c).or_default();
            if node.is_empty {
                return false
            }
        }
        true
    }
}

#[cfg(test)] // TODO: look up what this annotation means
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