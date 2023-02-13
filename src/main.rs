
// Declare the module we will use
mod structs;
mod linked_struct;
mod file_ops;
// Import from said module ( we are need a mod.rs file in structs to declare what is exposed)
use structs::node::ListNode;

fn main() {
    // linked_struct::run();
    file_ops::run();
}
