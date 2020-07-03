use heapsize::HeapSizeOf;

mod trie_v1;
mod trie_v2;

/**
* Benchmarking function for trie_v1
**/
pub fn trie1() {
    let mut obj = trie_v1::Trie::new();
    obj.insert("test");
    obj.insert("best");
    obj.insert("tes");
    obj.insert("testy");
    println!("Heap size: {}", obj.heap_size_of_children());
}

/**
* Benchmarking function for trie_v2
**/
pub fn trie2() {
    let mut obj = trie_v2::Trie::new();
    obj.insert("test");
    obj.insert("best");
    obj.insert("tes");
    obj.insert("testy");
    println!("Heap size: {}", obj.heap_size_of_children());
}

/**
* More generic way of performing trie actions
* Still need to figure out how to permit different struct types. Generics?
**/
fn do_action(obj: &mut trie_v1::Trie, action: &str, str: &str) -> bool {
    match action {
        "insert" => {
            obj.insert(str);
            true
        }
        "find" => obj.search(str),
        "pre" => obj.starts_with(str),
        _ => false,
    }
}

mod tests {
    #[test]
    fn test_do_action() {
        let mut obj = crate::trie_v1::Trie::new();
        let actions = [
            ("insert", "test"),
            ("find", "test"),
            ("pre", "to"),
            ("insert", "toronto"),
        ];
        for a in actions.iter() {
            super::do_action(&mut obj, a.0, a.1);
        }
        assert_eq!(super::do_action(&mut obj, "find", "test"), true)
    }
}
