use crate::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Body {
    pub node: Rc<RefCell<BodyNode>>,
}

impl Body {
    pub fn new() -> Body {
        let node = BodyNode { children: vec![] };
        Body {
            node: Rc::new(RefCell::new(node)),
        }
    }
}

pub struct BodyNode {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Node for BodyNode {
    fn render(&self) -> String {
        let inner = self
            .children
            .iter()
            .map(|x| x.borrow().render())
            .collect::<Vec<String>>()
            .join("\n");

        format!("<body>{}</body>", inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_document() {
        let body = Body::new();
        assert_eq!(body.node.borrow().render(), "<body></body>");
    }
}
