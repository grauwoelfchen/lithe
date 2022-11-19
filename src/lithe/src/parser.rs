use anyhow::Error;
use pest::Parser;
use pest::iterators::Pairs;

use crate::document::{Attr, Document, Element, NamedNodeMap};
use crate::document_type::DocumentType;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LitheParser;

pub fn parse(s: &str) -> Result<Document, Error> {
    let mut result = LitheParser::parse(Rule::document, s)?;

    let doc = Document::new();
    Ok(build(&mut result, 0, doc))
}

// TODO: renderer
fn build<'a>(
    pairs: &mut Pairs<'a, Rule>,
    level: usize,
    mut acc: Document<'a>,
) -> Document<'a> {
    for pair in pairs {
        let rule = pair.as_rule();
        let span = pair.as_span();

        // https://github.com/slim-template/slim/blob/master/test/literate/TESTS.md
        let mut inner = pair.into_inner();
        match rule {
            Rule::EOI => {
                return acc;
            }
            Rule::indent => {
                // TODO
                // Create tree structure
                // dbg!(&rule);
            }
            Rule::xml_doctype => {
                // TODO
                // <?xml version="1.0" encoding="utf-8">
                let doctype = DocumentType {
                    name: "xml".to_string(),
                    public_id: "",
                    system_id: "",
                };
                acc.r#type = Some(doctype);
            }
            Rule::xhtml_doctype => {
                let doctype = DocumentType::new("xhtml", span.as_str());
                acc.r#type = Some(doctype);
            }
            Rule::html_doctype => {
                let doctype = DocumentType::new("html", span.as_str());
                acc.r#type = Some(doctype);
            }
            Rule::comment => {
                let element = Element {
                    name: "",
                    parent: None,
                    children: vec![],
                    attributes: vec![],
                };
                acc.children.push(element);
            }
            Rule::html => {
                // global attributes
                let mut attributes: NamedNodeMap = vec![];
                for i in inner.clone() {
                    let mut ii = i.into_inner().take(2);
                    let name = if let Some(a) = ii.next() {
                        a.as_span().as_str()
                    } else {
                        break;
                    };
                    let value = ii.next().map_or("", |a| a.as_span().as_str());
                    attributes.push(Attr { name, value });
                }
                let element = Element {
                    name: "html",
                    parent: None,
                    children: vec![],
                    attributes,
                };
                acc.children.push(element);
            }
            Rule::head => {
                let element = Element {
                    name: "head",
                    parent: None,
                    children: vec![],
                    attributes: vec![],
                };
                acc.children.push(element);
            }
            Rule::body => {
                let element = Element {
                    name: "body",
                    parent: None,
                    children: vec![],
                    attributes: vec![],
                };
                acc.children.push(element);
            }
            _ => {} // do nothing
        }
        acc = build(&mut inner, level + 1, acc)
    }
    acc
}

#[cfg(test)]
mod test {
    use pest::Parser;
    use super::{LitheParser, Rule, parse};

    macro_rules! assert_rule {
        ($rule:expr, $input:expr) => {
            let result = LitheParser::parse($rule, $input)
                .unwrap()
                .peek()
                .unwrap()
                .as_rule();
            assert_eq!(result, $rule);
        };
    }

    #[test]
    fn test_code_comment() {
        let comments = vec![
            "/ foo bar baz qux quux",
            "/foo bar baz qux quux",
            "/  foo bar baz qux quux",
        ];
        for c in comments.iter() {
            assert_rule!(Rule::code_comment, c);
        }
    }

    #[test]
    fn test_html_comment() {
        let comments = vec![
            "/! foo bar baz qux quux",
            "/!foo bar baz qux quux",
            "/!  foo bar baz qux quux",
        ];
        for c in comments.iter() {
            assert_rule!(Rule::html_comment, c);
        }
    }

    #[test]
    fn test_doctype() {
        let doctypes = vec![
            "doctype xml",
            "doctype xml ISO-8859-1",
            "doctype html",
            "doctype  5",
            "doctype\n1.1",
            "doctype\n\n\n strict",
        ];
        for d in doctypes.iter() {
            assert_rule!(Rule::doctype, d);
        }
    }

    #[test]
    fn test_parse() {
        // TODO: test parse results
        assert!(parse("/ Foo\n").is_ok());
        assert!(parse("/! Bar").is_ok());

        assert!(parse("doctype xml").is_ok());
        assert!(parse("doctype  xml").is_ok());
        assert!(parse("doctype xml ISO-8859-1").is_ok());

        // TODO: mode

        // xhtml mode
        assert!(parse("doctype html").is_ok());
        assert!(parse("doctype 5").is_ok());
        assert!(parse("doctype 1.1").is_ok());
        assert!(parse("doctype strict").is_ok());
        assert!(parse("doctype frameset").is_ok());
        assert!(parse("doctype mobile").is_ok());
        assert!(parse("doctype basic").is_ok());
        assert!(parse("doctype transitional").is_ok());

        // html mode
        assert!(parse("doctype html").is_ok());
        assert!(parse("doctype 5").is_ok());
        assert!(parse("doctype strict").is_ok());
        assert!(parse("doctype frameset").is_ok());
        assert!(parse("doctype transitional").is_ok());

        assert!(parse("doctype unknown").is_err());
    }

    #[test]
    fn test_parse_empty_doc() {
        let doc = parse(
            r#"doctype html
/ Comment
/! Das ist ein Test
"#,
        )
        .unwrap();

        let doctype = doc.r#type.unwrap();
        assert_eq!(doctype.name, "html".to_string());
        assert_eq!(doctype.public_id, "");
        assert_eq!(doctype.system_id, "");

        assert!(doc.children.is_empty());
    }

    #[test]
    fn test_parse_html_tag() {
        let doc = parse(
            r#"doctype html
html
"#,
        )
        .unwrap();

        let html = &doc.children[0];
        assert_eq!(html.name, "html");

        assert!(html.children.is_empty());
        assert!(html.attributes.is_empty());
    }

    #[test]
    fn test_parse_html_tag_with_attributes() {
        let doc = parse(
            r#"doctype html
html lang=en
  head
"#,
        )
        .unwrap();

        let html = &doc.children[0];
        assert_eq!(html.name, "html");

        assert!(html.children.is_empty());

        let attr = &html.attributes[0];
        assert_eq!(attr.name, "lang");
        assert_eq!(attr.value, "en");
    }

    #[test]
    fn test_parse_head_tag() {
        let doc = parse(
            r#"doctype html
html
  head
"#,
        )
        .unwrap();

        let html = &doc.children[0];
        assert_eq!(html.name, "html");

        // TODO
        // dbg!(&html.children);
    }
}
