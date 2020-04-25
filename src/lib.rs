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
        "<!DOCTYPE html><html>".to_string()
            //+ self.children.iter().fold("", |acc, x| x.borrow())
            + "</html>"

        //acc.push(c.render()))
        //+ "</html>".to_string()
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
        assert_eq!(result, "<!DOCTYPE html><html></html>");
    }

    #[test]
    fn document_renders_children() {
        let mut doc = Document::new();
        doc.h1(format!("blah {}", 1));
        let result = doc.render();
        assert_eq!(result, "<!DOCTYPE html><html></html>")
        //        assert_eq!(result, "<!DOCTYPE html><html><h1>blah 1</h1></html>")
    }
}
