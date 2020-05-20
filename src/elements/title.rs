use crate::Node;

pub struct Title {
    pub text: String,
}

impl Node for Title {
    fn render(&self) -> String {
        format!("<title>{}</title>", self.text)
    }
}

impl Title {
    pub fn new(text: String) -> Title {
        Title { text }
    }
}
