use crate::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct H1 {
    text: Rc<RefCell<String>>,
}

impl Node for H1 {
    fn render(&self) -> String {
        format!("<h1>{}</h1>", self.text.borrow())
    }
}

impl H1 {
    pub fn new(text: String) -> H1 {
        H1 {
            text: Rc::new(RefCell::new(text)),
        }
    }

    pub fn title(&mut self, text: String) {
        let mut inner = self.text.borrow_mut();
        inner.clear();
        inner.push_str(&text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let h1 = H1::new("test".to_owned());
        assert_eq!(h1.render(), "<h1>test</h1>");
    }
}
