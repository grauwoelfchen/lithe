use crate::dtd::DTD;

// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType
pub struct DocumentType {
    pub name: String,
    pub public_id: &'static str,
    pub system_id: &'static str,
}

impl DocumentType {
    pub fn new(r#type: &'static str, name: &str) -> Self {
        let dtd = DTD::new(r#type, name);
        Self {
            name: name.to_string(),
            public_id: dtd.public_id(),
            system_id: dtd.system_id(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_document_type_new() {
        let doctype = DocumentType::new("unknown", "invalid");
        assert_eq!(doctype.name, "invalid");
        assert_eq!(doctype.public_id, "");
        assert_eq!(doctype.system_id, "");

        let doctype = DocumentType::new("unknown", "strict");
        assert_eq!(doctype.name, "strict");
        assert_eq!(doctype.public_id, "");
        assert_eq!(doctype.system_id, "");

        let doctype = DocumentType::new("xhtml", "invalid");
        assert_eq!(doctype.name, "invalid");
        assert_eq!(doctype.public_id, "");
        assert_eq!(doctype.system_id, "");

        let doctype = DocumentType::new("xhtml", "frameset");
        assert_eq!(doctype.name, "frameset");
        assert_eq!(doctype.public_id, "-//W3C//DTD XHTML 1.0 Frameset//EN");
        assert_eq!(
            doctype.system_id,
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd"
        );

        let doctype = DocumentType::new("html", "invalid");
        assert_eq!(doctype.name, "invalid");
        assert_eq!(doctype.public_id, "");
        assert_eq!(doctype.system_id, "");

        let doctype = DocumentType::new("html", "transitional");
        assert_eq!(doctype.name, "transitional");
        assert_eq!(doctype.public_id, "-//W3C//DTD HTML 4.01 Transitional//EN");
        assert_eq!(doctype.system_id, "http://www.w3.org/TR/html4/loose.dtd");
    }
}
