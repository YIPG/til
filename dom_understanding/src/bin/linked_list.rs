pub struct ListNode {
  val: i32,
  next: Option<Box<NListNode>>
}

impl ListNode {
  pub fn new(val: i32) -> Self{
    Node {
      val,
      next: None
    }
  }
}

pub fn from_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
  let mut current = None;
  for &v in vec.iter().rev() {
    let mut node = ListNode.new(v);
    node.next = current;
    current = Some(Box::new(node));
  }
  current
}