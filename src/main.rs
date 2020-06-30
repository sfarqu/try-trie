use heapsize::HeapSizeOf;

mod trie_v1;
mod trie_v2;

fn main() {
    let mut obj1 = trie_v1::Trie::new();
    obj1.insert("test");
    // obj1.insert("tes");
    // obj1.insert("testy");
    println!("Heap size: {}", obj1.heap_size_of_children());

    let mut obj2 = trie_v2::Trie::new();
    obj2.insert("test");
    obj2.insert("best");
    // obj1.insert("tes");
    // obj1.insert("testy");
    println!("Heap size: {}", obj2.heap_size_of_children());
}
