use crate::document_type::DocumentType;

#[derive(Debug)]
pub struct Attr<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

pub type HTMLCollection<'a> = Vec<Element<'a>>;
pub type NamedNodeMap<'a> = Vec<Attr<'a>>;

// https://developer.mozilla.org/en-US/docs/Web/API/Element
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
#[derive(Debug)]
pub struct Element<'a> {
    pub name: &'static str,
    pub attributes: NamedNodeMap<'a>,
    pub parent: Option<&'a Element<'a>>,
    pub children: HTMLCollection<'a>,
}

impl<'a> Element<'a> {
    pub fn new() -> Self {
        Self {
            name: "",
            attributes: vec![],
            parent: None,
            children: vec![],
        }
    }
}

impl<'a> Default for Element<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// https://developer.mozilla.org/en-US/docs/Web/API/Document
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
#[derive(Debug)]
pub struct Document<'a> {
    pub r#type: Option<DocumentType>,
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
}
