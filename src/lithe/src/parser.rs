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

    let doc = build(&mut result);
    Ok(doc)
}

/// Builds structured tree data.
///
/// At the moment, this should look like:
///
/// ```rust
/// let input = r#"doctype html
/// html
///   head
///     link rel="stylesheet" href="style.css"
///   body
/// "#;
/// ```
///
/// ```txt
/// [src/lithe/src/parser.rs:16] &doc = Document {
///    type: Some(
///        DocumentType {
///            dtd: DTD {
///                spec: "html",
///                name: "html",
///            },
///            name: "html",
///            public_id: "",
///            system_id: "",
///        },
///    ),
///    children: [
///        Element {
///            name: "html",
///            attributes: [],
///            children: [
///                Element {
///                    name: "head",
///                    attributes: [],
///                    children: [
///                        Element {
///                            name: "link",
///                            attributes: [
///                                Attr {
///                                    name: "rel",
///                                    value: "stylesheet",
///                                },
///                                Attr {
///                                    name: "href",
///                                    value: "style.css",
///                                },
///                            ],
///                            children: [],
///                        },
///                    ],
///                },
///                Element {
///                    name: "body",
///                    attributes: [],
///                    children: [],
///                },
///            ],
///        },
///    ],
///}
/// ```
fn build<'a>(pairs: &mut Pairs<'a, Rule>) -> Document<'a> {
    let mut doc = Document::new();

    #[allow(clippy::useless_conversion)]
    for pair in pairs.into_iter() {
        let rule = pair.as_rule();
        let inner = pair.into_inner();
        match rule {
            Rule::EOI => {
                return doc;
            }
            Rule::doctype => {
                for i in inner {
                    if i.as_rule() == Rule::doctype_value {
                        // TODO: mode (html|xhtml)
                        let (spec, name) = match i.as_span().as_str() {
                            "html" => ("html", "html"),
                            "5" => ("html", "5"),
                            _ => ("", ""),
                        };
                        let doctype = DocumentType::new(spec, name);
                        doc.r#type = Some(doctype);
                        // TODO: Is there any way? (instead of reusing pairs)
                        doc.children = build_element(pairs, 0);
                        break;
                    }
                }
                return doc;
            }
            _ => {}
        }
    }
    doc
}

fn build_attributes<'a>(pairs: &mut Pairs<'a, Rule>) -> Vec<Attr<'a>> {
    let mut attributes: NamedNodeMap = vec![];

    for pair in pairs {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        match rule {
            Rule::link_attribute => {
                // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link
                // NOTE: array_chunks() or next_chunk()?
                while let Some(i) = inner.next() {
                    let name = i.as_span().as_str();
                    let value =
                        inner.next().map_or("", |a| a.as_span().as_str());
                    attributes.push(Attr { name, value });
                }
            }
            _ => {
                // global attributes
                let mut i = inner.take(2);
                let name = if let Some(a) = i.next() {
                    a.as_span().as_str()
                } else {
                    break;
                };
                let value = i.next().map_or("", |a| a.as_span().as_str());
                attributes.push(Attr { name, value });
            }
        }
    }
    attributes
}

fn build_element<'a>(
    pairs: &mut Pairs<'a, Rule>,
    level: usize,
) -> Vec<Element<'a>> {
    let mut result = vec![];
    for pair in pairs {
        let rule = pair.as_rule();

        match rule {
            Rule::EOI => {
                return result;
            }
            Rule::indent => {
                // FIXME: for stacktrace?
                // let span = pair.as_span();
                // let indent = span.end() - span.start();
                // dbg!(&indent);
            }
            Rule::comment => {
                let element = Element {
                    name: "".to_string(),
                    children: vec![],
                    attributes: vec![],
                };
                result.push(element);
            }
            Rule::html | Rule::head | Rule::body => {
                // block element
                let name = format!("{:?}", rule);
                let mut element = Element {
                    name,
                    children: vec![],
                    attributes: vec![],
                };
                let mut inner = pair.into_inner();
                element.attributes = build_attributes(&mut inner);
                element.children = build_element(&mut inner, level);
                result.push(element);
            }
            Rule::link => {
                // void element
                let mut element = Element {
                    name: "link".to_string(),
                    children: vec![],
                    attributes: vec![],
                };
                let mut inner = pair.into_inner();
                element.attributes = build_attributes(&mut inner);
                result.push(element);
            }
            _ => {} // do nothing
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_rule {
        ($rule:expr, $input:expr) => {
            let result = LitheParser::parse($rule, $input)
                .unwrap()
                .peek()
                .unwrap()
                .as_rule();
            assert_eq!($rule, result);
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
        assert_eq!("html".to_string(), doctype.name);
        assert_eq!("", doctype.public_id);
        assert_eq!("", doctype.system_id);

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
        assert_eq!("html", html.name);

        assert!(html.children.is_empty());
        assert!(html.attributes.is_empty());
    }

    #[test]
    fn test_parse_html_tag_with_attributes() {
        let doc = parse(
            r#"doctype html
html lang="en"
"#,
        )
        .unwrap();

        let html = &doc.children[0];
        assert_eq!("html", html.name);

        assert!(html.children.is_empty());

        let attr = &html.attributes[0];
        assert_eq!("lang", attr.name);
        assert_eq!("en", attr.value);
    }

    #[test]
    fn test_parse_entire_doc() {
        let doc = parse(
            r#"doctype html
html
  head
    link rel="stylesheet" href="style.css"
  body
"#,
        )
        .unwrap();

        let doctype = DocumentType::new("html", "html");
        let expected = Document {
            r#type: Some(doctype),
            children: vec![Element {
                name: "html".to_string(),
                attributes: vec![],
                children: vec![
                    Element {
                        name: "head".to_string(),
                        attributes: vec![],
                        children: vec![Element {
                            name: "link".to_string(),
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
                            children: vec![],
                        }],
                    },
                    Element {
                        name: "body".to_string(),
                        attributes: vec![],
                        children: vec![],
                    },
                ],
            }],
        };
        assert_eq!(expected, doc);
    }
}
