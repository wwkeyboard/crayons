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
