use anyhow::Error;

use crate::document::Document;

pub fn render(document: &Document) -> Result<String, Error> {
    let mut result = "".to_string();
    let document_type = &document.r#type;
    if let Some(v) = document_type {
        result.push_str(&v.as_tag());

        for e in &document.children {
            result.push_str(&e.as_tag());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::document::{Attr, Element};
    use crate::document_type::DocumentType;

    #[test]
    fn test_render_empty_children() {
        let doc = Document::new();
        assert_eq!("".to_string(), render(&doc).unwrap());

        let doc_type = DocumentType::new("html", "5");
        let doc = Document {
            r#type: Some(doc_type),
            children: vec![],
        };
        assert_eq!(render(&doc).unwrap(), "<!DOCTYPE HTML>".to_string());
    }

    #[test]
    fn test_render_with_children() {
        let doc = Document::new();
        assert_eq!("".to_string(), render(&doc).unwrap());

        let link = Element {
            name: "link",
            attributes: vec![
                Attr {
                    name: "rel",
                    value: "stylesheet",
                },
                Attr {
                    name: "href",
                    value: "style.css",
                },
            ],
            parent: None,
            children: vec![],
        };

        let head = Element {
            name: "head",
            attributes: vec![],
            parent: None,
            children: vec![link],
        };

        let html = Element {
            name: "html",
            attributes: vec![Attr {
                name: "lang",
                value: "en",
            }],
            parent: None,
            children: vec![head],
        };

        let doc_type = DocumentType::new("html", "5");
        let doc = Document {
            r#type: Some(doc_type),
            children: vec![html],
        };
        assert_eq!(
            render(&doc).unwrap(),
            inline!(
                r#"<!DOCTYPE HTML>
<html lang="en">
<head>
<link rel="stylesheet" href="style.css" />
</head>
</html>
"#
            )
        );
    }
}
