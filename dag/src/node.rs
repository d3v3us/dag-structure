#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            val: self.val,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}