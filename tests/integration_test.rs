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
    let document = Document::from_read(html.as_bytes()).unwrap();
    let title = document.find(Name("title")).last().unwrap();
    assert_eq!("test", title.text());
}

#[test]
fn test_simple_blog_post() {
    let mut doc = cDoc::new();
    doc.head.title.text = "My Blog Post".to_string();

    let mut h1 = doc.body.h1();
    h1.title("something".to_string());

    //    let mut post = doc.body.div();
    //    post.text

    // parse the document to check for errors
    let _ = Document::from(doc.render().as_str());
}
