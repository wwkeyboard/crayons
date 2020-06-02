use crate::Node;
use std::cell::RefCell;
use std::rc::Rc;

use crate::elements::h1::H1;

pub struct Body {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Body {
    pub fn new() -> Body {
        Body { children: vec![] }
    }

    pub fn h1(&mut self) -> H1 {
        let h1 = H1::new("".to_owned());
        self.children.push(Rc::new(RefCell::new(h1.clone())));
        h1
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

    #[test]
    fn bare_heading() {
        let mut body = Body::new();
        body.h1().title("this thing!".to_string());
        assert_eq!(body.render(), "<body><h1>this thing!</h1></body>");
    }
}
