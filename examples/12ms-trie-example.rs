/**
* Example code with a faster runtime on leetcode
**/

#[derive(Debug)]
struct Trie {
    root: Box<Node>, // root is pointer to Node object on heap, prevents mutable self prob in my version
}

#[derive(Debug, Default)]
struct Node {
    /// A pointer to each child node, one for each lowercase character
    children: Vec<Option<Box<Node>>>, // use vector instead of dictionary, keys are char codes
    
    /// This is a terminal node
    terminal: bool,
}

impl Node {
    fn new() -> Self {
        let mut children = Vec::new();
        children.reserve(26);
        
        for _ in 0..26 { // does this init 27 values? NO - range is exclusive of last number
            children.push(None); // initialize all values to None--what is default Vec init value?
        }
        
        Node {
            children: children,
            terminal: false,
        }
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie { root: Box::new(Node::new()) }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root;
        
        for c in word.chars() {
            let i = (c as usize - 97);

            if curr_node.children[i].is_none() {
                // Insert a new node into the trie for this character
                curr_node.children[i] = Some(Box::new(Node::new()));
            }

            curr_node = curr_node.children[i].as_mut().unwrap();
        }
        
        // Mark the last character's node as "terminal" to end the word
        curr_node.terminal = true;
    }
    
    fn search_internal(&self, word: String, full: bool) -> bool {
        let mut curr_node = &self.root;
        
        for c in word.chars() {
            let i = (c as usize - 97); // cheater way of getting 0-26, only works lc ASCII

            if curr_node.children[i].is_none() {
                // We did not find this character in the trie
                return false;
            }
            
            curr_node = &curr_node.children[i].as_ref().unwrap();
        }
        
        // Return "found" if and only if this is a terminal node in the trie
        !full || curr_node.terminal // this !full is hard for me to logic around, is this good practice?
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.search_internal(word, true)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.search_internal(prefix, false)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

// This is only here to make the compiler happy, this code never runs
fn main() {
    println!("Hello, world!");
}