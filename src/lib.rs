use std::cell::RefCell;
use std::rc::Rc;

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
        let h = Rc::new(RefCell::new(H1Node { text: text }));
        self.children.push(h.clone());
        H1 { node: h }
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
            "<!DOCTYPE html><head><meta charset=\"UTF-8\"></head><html>{}</html>",
            inner
        )
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn blank_document() {
        let doc = Document::new();
        let result = Document::render(&doc);
        assert_eq!(
            result,
            "<!DOCTYPE html><head><meta charset=\"UTF-8\"></head><html></html>"
        );
    }

    #[test]
    fn document_renders_children() {
        let mut doc = Document::new();
        doc.h1(format!("blah {}", 1));
        let result = doc.render();
        assert_eq!(
            result,
            "<!DOCTYPE html><head><meta charset=\"UTF-8\"></head><html><h1>blah 1</h1></html>"
        )
    }
}
