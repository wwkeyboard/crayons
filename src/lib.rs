use std::cell::RefCell;
use std::rc::Rc;

use elements::h1::H1;

mod elements;

pub trait Node {
    fn render(&self) -> String;
}

pub struct Document {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Document {
    pub fn new() -> Document {
        Document { children: vec![] }
    }

    pub fn h1(&mut self, text: String) -> H1 {
        let (element, node) = H1::new(text);
        self.children.push(node);
        element
    }
}

impl Node for Document {
    fn render(&self) -> String {
        let inner = self
            .children
            .iter()
            .map(|x| x.borrow().render())
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body>{}</body></html>",
            inner
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_document() {
        let doc = Document::new();
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body></body></html>"
        );
    }

    #[test]
    fn document_renders_children() {
        let mut doc = Document::new();
        doc.h1(format!("blah {}", 1));
        let result = doc.render();
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"></head><body><h1>blah 1</h1></body></html>"
        )
    }
}
