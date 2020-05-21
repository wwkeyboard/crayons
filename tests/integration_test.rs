use crayons::Document as cDoc;
use select::document::Document;
use select::predicate::Name;

#[test]
fn test_empty_document() {
    let mut doc = cDoc::new();

    doc.head.title.text = "test".to_owned();

    let html = &doc.render()[..];

    let document = Document::from(html);

    let title = document.find(Name("title")).last().unwrap();

    assert_eq!("test", title.text());
}
