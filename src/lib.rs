#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Node {
    val: i32,
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            right: None,
            left: None,
        }
    }
    pub fn insert(&mut self, val: i32) {
        if val > self.val {
            if let Some(right) = &mut self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(Node::new(val)));
                return;
            }
        }
        if let Some(left) = &mut self.left {
            left.insert(val);
        } else {
            self.left = Some(Box::new(Node::new(val)));
        }
    }
    pub fn search(&self, val: i32) -> Vec<&'static str> {
        let mut result: Vec<&'static str> = vec![];
        loop {
            if val == self.val {
                result.push("found");
                break;
            }
            if val > self.val {
                if self.right.is_some() {
                    result.push("right");
                } else {
                    result.push("not found");
                    break;
                }
            }
            if self.left.is_some() {
                result.push("left");
            } else {
                result.push("not found");
                break;
            }
        }
        result
    }
    pub fn insert_vec(&mut self, vec: Vec<i32>) {
        for val in vec {
            self.insert(val);
        }
    }
}
