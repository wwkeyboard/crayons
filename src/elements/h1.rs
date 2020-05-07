use std::cell::RefCell;
use std::rc::Rc;

use crate::Node;

pub struct H1Node {
    text: String,
}

impl Node for H1Node {
    fn render(&self) -> String {
        format!("<h1>{}</h1>", self.text)
    }
}

pub struct H1 {
    node: Rc<RefCell<H1Node>>,
}

impl H1 {
    pub fn new(text: String) -> (H1, Rc<RefCell<H1Node>>) {
        let node = Rc::new(RefCell::new(H1Node { text: text }));
        let element = H1 { node: node.clone() };
        (element, node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let (test_element, test_node) = H1::new("test".to_owned());
        assert_eq!(test_node.borrow().render(), "<h1>test</h1>");
    }
}
