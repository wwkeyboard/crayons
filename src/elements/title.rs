use std::cell::RefCell;
use std::rc::Rc;

use crate::Node;

pub struct TitleNode {
    text: String,
}

impl Node for TitleNode {
    fn render(&self) -> String {
        format!("<title>{}</title>", self.text)
    }
}

pub struct Title {
    node: Rc<RefCell<TitleNode>>,
}

impl Title {
    pub fn new(text: String) -> (Title, Rc<RefCell<TitleNode>>) {
        let node = Rc::new(RefCell::new(TitleNode { text: text }));
        let element = Title { node: node.clone() };
        (element, node)
    }
}
