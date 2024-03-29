use crate::document_type::DocumentType;

#[derive(Debug, Eq, PartialEq)]
pub struct Attr<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub type HTMLCollection<'a> = Vec<Element<'a>>;
pub type NamedNodeMap<'a> = Vec<Attr<'a>>;

// https://developer.mozilla.org/en-US/docs/Web/API/Element
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
#[derive(Debug, Eq, PartialEq)]
pub struct Element<'a> {
    pub name: String,
    pub attributes: NamedNodeMap<'a>,
    pub children: HTMLCollection<'a>,
}

// https://html.spec.whatwg.org/multipage/syntax.html#void-elements
const VOID_ELEMENTS: [&str; 13] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
    "source", "track", "wbr",
];

impl<'a> Element<'a> {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn as_tag(&self) -> String {
        let mut out = format!("<{}", self.name);
        if !self.attributes.is_empty() {
            out.push(' ');
            out.push_str(
                &self
                    .attributes
                    .iter()
                    .map(|a| format!("{}=\"{}\"", a.name, a.value))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
        if VOID_ELEMENTS.contains(&self.name.as_str()) {
            out.push_str(" />");
        } else {
            out.push('>');
            for c in &self.children {
                out.push_str(&c.as_tag());
            }
            out.push_str(&format!("</{}>", self.name));
        }
        out
    }
}

impl<'a> Default for Element<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Document
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
#[derive(Debug, Eq, PartialEq)]
pub struct Document<'a> {
    pub r#type: Option<DocumentType<'a>>,
    pub children: Vec<Element<'a>>,
}

impl<'a> Document<'a> {
    pub fn new() -> Self {
        Self {
            r#type: None,
            children: vec![],
        }
    }
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_document_new() {
        let doc = Document::new();
        assert!(doc.r#type.is_none());
        assert!(doc.children.is_empty());
    }

    #[test]
    fn test_document_default() {
        let doc = Document::default();
        assert!(doc.r#type.is_none());
        assert!(doc.children.is_empty());
    }

    #[test]
    fn test_document_equality() {
        let a = Document::default();
        let b = Document::default();
        assert_eq!(a, b);
    }

    #[test]
    fn test_element_equality() {
        let a = Element::default();
        let b = Element::default();
        assert_eq!(a, b);
    }

    #[test]
    fn test_attr_equality() {
        let a = Attr {
            name: "",
            value: "",
        };
        let b = Attr {
            name: "",
            value: "",
        };
        assert_eq!(a, b);
    }
}
