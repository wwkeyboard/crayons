use std::cell::RefCell;
use std::rc::Rc;

use elements::h1::H1;
use elements::head::Head;
use elements::title::Title;

mod elements;

pub trait Node {
    fn render(&self) -> String;
}

pub struct Document {
    head: Rc<RefCell<dyn Node>>,
    body: Vec<Rc<RefCell<dyn Node>>>,
}

impl Document {
    pub fn new() -> (Head, Document) {
        let (head, head_node) = Head::new();
        (
            head,
            Document {
                body: vec![],
                head: head_node,
            },
        )
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
        let head = self.head.borrow().render();
        let inner = self
            .body
            .iter()
            .map(|x| x.borrow().render())
            .collect::<Vec<String>>()
            .join("\n");
        format!("<!DOCTYPE html><html>{}<body>{}</body></html>", head, inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_document() {
        let (head, doc) = Document::new();
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title></title></head><body></body></html>"
        );
    }

    #[test]
    fn with_body() {
        let (mut head, mut doc) = Document::new();
        head.title("this test!".to_owned());
        doc.h1("I'm here!".to_owned());
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>this test!</title></head><body><h1>I'm here!</h1></body></html>"
        );
    }
}
