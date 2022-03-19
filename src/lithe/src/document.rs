pub struct Attr {
    pub name: String,
    pub value: String,
}

pub type HTMLCollection = Vec<Element>;
pub type NamedNodeMap = Vec<Attr>;

// https://developer.mozilla.org/en-US/docs/Web/API/Element
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html
pub struct Element {
    pub name: &'static str,
    pub attributes: NamedNodeMap,
    pub children: HTMLCollection,
}

// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType
pub struct DocumentType {
    pub name: String,
    pub public_id: &'static str,
    pub system_id: &'static str,
}

// https://developer.mozilla.org/en-US/docs/Web/API/Document
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
pub struct Document {
    pub r#type: Option<DocumentType>,
    pub children: Vec<Element>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            r#type: None,
            children: vec![],
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
