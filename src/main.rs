use heapsize::HeapSizeOf;

mod trie_v1;
mod trie_v2;

fn main() {
    let mut obj1 = trie_v1::Trie::new();
    obj1.insert("test");
    // obj1.insert("tes");
    // obj1.insert("testy");
    println!("Heap size: {}", obj1.heap_size_of_children());
}
