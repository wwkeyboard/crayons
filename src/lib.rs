#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

use elements::h1::H1;
use elements::head::Head;

mod elements;

pub trait Node {
    fn render(&self) -> String;
}

pub struct Document {
    pub head: Head,
    body: Vec<dyn Node>,
}

impl Document {
    pub fn new() -> Document {
        let head = Head::new();
        Document {
            body: vec![],
            head: head,
        }
    }

    pub fn h1(&mut self, text: String) -> H1 {
        let node = H1::new(text);
        self.body.push(node);
        node
    }

    pub fn render(&self) -> String {
        let head = self.head.render();
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
        doc.head.title.text = "this test!".to_owned();
        doc.h1("I'm here!".to_owned());
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>this test!</title></head><body><h1>I'm here!</h1></body></html>"
        );
    }
}
