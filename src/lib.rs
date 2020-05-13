use std::cell::RefCell;
use std::rc::Rc;

use elements::h1::H1;
use elements::title::Title;
use elements::head::Head;

mod elements;

pub trait Node {
    fn render(&self) -> String;
}

pub struct Document {
    body: Vec<Rc<RefCell<dyn Node>>>,
}

impl Document {
    pub fn new() -> Document {
        Document { body: vec![] }
    }

    pub fn h1(&mut self, text: String) -> H1 {
        let (element, node) = H1::new(text);
        self.body.push(node);
        element
    }

    pub fn title(&mut self, text: String) -> Title {
        let (element, node) = Title::new(text);
        self.body.push(node);
        element
    }
}

impl Node for Document {
    fn render(&self) -> String {
        let inner = self
            .body
            .iter()
            .map(|x| x.borrow().render())
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title></title></head><body>{}</body></html>",
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
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title></title></head><body></body></html>"
        );
    }

    #[test]
    fn with_body() {
        let mut doc = Document::new();
        doc.h1("I'm here!".to_owned());
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title></title></head><body><h1>I'm here!</h1></body></html>"
        );
    }
}
