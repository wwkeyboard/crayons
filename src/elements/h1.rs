use crate::Node;

pub struct H1 {
    pub text: RC<RefCell<String>>,
}

impl Node for H1 {
    fn render(&self) -> String {
        format!("<h1>{}</h1>", self.text)
    }
}

impl H1 {
    pub fn new(text: String) -> H1 {
        H1 { text: text }
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
