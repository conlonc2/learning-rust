
// Declare the module we will use
mod structs;
// Import from said module ( we are need a mod.rs file in structs to declare what is exposed)
use structs::node::ListNode;

fn main() {
    let mut root: ListNode = ListNode {
        value: 1,
        l: None,
        r: None,
    };
    root.add_node(ListNode {value: 1, l: None, r: None});
    root.add_node(ListNode {value: 3, l: None, r: None});
    root.add_node(ListNode {value: 23, l: None, r: None});
    root.add_node(ListNode {value: 53, l: None, r: None});
    root.add_node(ListNode {value: 73, l: None, r: None});
    root.add_node(ListNode {value: 663, l: None, r: None});
    println!("ListNode: {:?}", root);
}
