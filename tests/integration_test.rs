use crayons::Document as cDoc;
use select::document::Document;
use select::predicate::Name;

#[test]
fn test_empty_document() {
    // Build a new document with only a title
    let mut doc = cDoc::new();
    doc.head.title.text = "test".to_owned();

    // render the document
    let html = doc.render();

    // parse the document and test the results
    let document = Document::from(html.as_str());
    let title = document.find(Name("title")).last().unwrap();
    assert_eq!("test", title.text());
}
