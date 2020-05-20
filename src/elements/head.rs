use crate::elements::title::Title;
use crate::Node;

pub struct Head {
    pub title: Title,
}

impl Node for Head {
    fn render(&self) -> String {
        format!(
            "<head><meta charset=\"UTF-8\">{}</head>",
            self.title.render()
        )
    }
}

impl Head {
    pub fn new() -> Head {
        Head {
            title: Title::new("".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut node = Head::new();
        assert_eq!(
            node.render(),
            "<head><meta charset=\"UTF-8\"><title></title></head>"
        );
        node.title.text = "test".to_string();
        assert_eq!(
            node.render(),
            "<head><meta charset=\"UTF-8\"><title>test</title></head>"
        );
    }
}
