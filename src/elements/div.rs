use std::cell::RefCell;
use std::rc::Rc;

use crate::elements::h1::H1;
use crate::Node;

#[derive(Clone)]
pub struct Div {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl Node for Div {
    fn render(&self) -> String {
        let inner = self
            .children
            .iter()
            .map(|x| x.borrow().render())
            .collect::<Vec<String>>()
            .join("\n");

        format!("<div>{}</div>", inner)
    }
}

impl Div {
    pub fn new() -> Div {
        Div { children: vec![] }
    }

    fn h1(&mut self) -> H1 {
        let h1 = H1::new("".to_owned());
        self.children.push(Rc::new(RefCell::new(h1.clone())));
        h1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptydiv() {
        let div = Div { children: vec![] };
        assert_eq!(div.render(), "<div></div>");
    }

    #[test]
    fn with_header() {
        let mut div = Div { children: vec![] };
        div.h1().title("hi".to_string());
        assert_eq!(div.render(), "<div><h1>hi</h1></div>");
    }
}
