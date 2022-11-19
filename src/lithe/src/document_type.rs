use crate::dtd::DTD;

// https://developer.mozilla.org/en-US/docs/Web/API/DocumentType
#[derive(Clone, Debug)]
pub struct DocumentType<'a> {
    dtd: DTD<'a>,
    pub name: &'a str,
    pub public_id: &'static str,
    pub system_id: &'static str,
}

impl<'a> DocumentType<'a> {
    pub fn new(r#type: &'static str, name: &'a str) -> Self {
        let dtd = DTD::new(r#type, name);
        let public_id = dtd.public_id();
        let system_id = dtd.system_id();

        Self {
            dtd,
            name,
            public_id,
            system_id,
        }
    }

    pub fn as_tag(&self) -> String {
        let mut out = "<!DOCTYPE".to_string();
        let dec = match self.dtd.spec {
            "html" => match self.dtd.name {
                "5" => " HTML>".to_string(),
                _ => format!(
                    " HTML PUBLIC \"{}\" \"{}\">",
                    self.public_id, self.system_id
                ),
            },
            "xhtml" => format!(
                " html PUBLIC \"{}\" \"{}\">",
                self.public_id, self.system_id
            ),
            _ => "".to_string(),
        };
        out.push_str(&dec);
        out
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

    #[test]
    fn test_as_tag() {
        let doctype = DocumentType::new("html", "5");
        assert_eq!(doctype.as_tag(), "<!DOCTYPE HTML>");

        let doctype = DocumentType::new("html", "strict");
        assert_eq!(
            doctype.as_tag(),
            inline!(
                r#"<!DOCTYPE
 HTML
 PUBLIC
 "-//W3C//DTD HTML 4.01//EN"
 "http://www.w3.org/TR/html4/strict.dtd"
>"#
            )
        );

        let doctype = DocumentType::new("html", "frameset");
        assert_eq!(
            doctype.as_tag(),
            inline!(
                r#"<!DOCTYPE
 HTML
 PUBLIC
 "-//W3C//DTD HTML 4.01 Frameset//EN"
 "http://www.w3.org/TR/html4/frameset.dtd"
>"#
            )
        );

        let doctype = DocumentType::new("html", "transitional");
        assert_eq!(
            doctype.as_tag(),
            inline!(
                r#"<!DOCTYPE
 HTML
 PUBLIC
 "-//W3C//DTD HTML 4.01 Transitional//EN"
 "http://www.w3.org/TR/html4/loose.dtd"
>"#
            )
        );
    }
}
