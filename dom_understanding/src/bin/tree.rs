#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub previous: Option<Box<TreeNode>>,
    pub next: Option<Box<TreeNode>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) Self{
        TreeNode{
            val,
            left: None,
            right, None
        }
    }
}

pub fn main() {

}