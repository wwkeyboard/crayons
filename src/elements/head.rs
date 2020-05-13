use std::cell::RefCell;
use std::rc::Rc;

use crate::Node;

pub struct HeadNode {
    title: String,
}

impl Node for HeadNode {
    fn render(&self) -> String {
        format!("<head><meta charset=\"UTF-8\"><title>{}</title></head>", self.title)
    }
}

pub struct Head {
    node: Rc<RefCell<HeadNode>>,
}

impl Head {
    pub fn new(title: String) -> (Head, Rc<RefCell<HeadNode>>) {
        let node = Rc::new(RefCell::new(HeadNode { title: title }));
        let element = Head { node: node.clone() };
        (element, node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let (test_element, test_node) = Head::new("test".to_owned());
        assert_eq!(test_node.borrow().render(), "<head><meta charset=\"UTF-8\"><title>test</title></head>");
    }
}
