use crate::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Body {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Body {
    pub fn new() -> Body {
        Body { children: vec![] }
    }
}

impl Node for Body {
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
        assert_eq!(body.render(), "<body></body>");
    }
}
