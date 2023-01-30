#[derive(Debug)]
pub struct ListNode {
    pub value: u32,
    pub l: Option<Box<ListNode>>,
    pub r: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn add_node(&mut self, node: ListNode) {
        let mut cnode: &mut ListNode = self;
        if node.value > 50 {
            while cnode.l.is_some() {
                cnode = cnode.l.as_mut().unwrap();
            }
            cnode.l = Some(Box::new(node));
        } else {
            while cnode.r.is_some() {
              cnode = cnode.r.as_mut().unwrap();
            }
            cnode.r = Some(Box::new(node));
        }
    }
}
