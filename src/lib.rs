#![allow(dead_code)]

use elements::body::Body;
use elements::head::Head;

mod elements;

pub trait Node {
    fn render(&self) -> String;
}

pub struct Document {
    pub head: Head,
    body: Body,
}

impl Document {
    pub fn new() -> Document {
        Document {
            body: Body::new(),
            head: Head::new(),
        }
    }

    pub fn render(&self) -> String {
        let head = self.head.render();
        let body = self.body.render();
        format!("<!DOCTYPE html><html>{}<body>{}</body></html>", head, body)
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
        doc.body.h1().title("I'm here!".to_owned());
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>this test!</title></head><body><h1>I'm here!</h1></body></html>"
        );
    }
}
