use std::cell::RefCell;
use std::rc::Rc;

use crate::Node;

pub struct HeadNode {
    pub title: String,
}

impl Node for HeadNode {
    fn render(&self) -> String {
        format!(
            "<head><meta charset=\"UTF-8\"><title>{}</title></head>",
            self.title
        )
    }
}

pub struct Head {
    node: Rc<RefCell<HeadNode>>,
}

impl Head {
    pub fn new() -> (Head, Rc<RefCell<HeadNode>>) {
        let node = Rc::new(RefCell::new(HeadNode {
            title: "".to_owned(),
        }));
        let element = Head { node: node.clone() };
        (element, node)
    }

    pub fn title(&mut self, title: String) {
        self.node.borrow_mut().title = title;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let (mut test_element, test_node) = Head::new();
        assert_eq!(
            test_node.borrow().render(),
            "<head><meta charset=\"UTF-8\"><title></title></head>"
        );
        test_element.title("test".to_string());
        assert_eq!(
            test_node.borrow().render(),
            "<head><meta charset=\"UTF-8\"><title>test</title></head>"
        );
    }
}
