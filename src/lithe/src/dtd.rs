use std::collections::HashMap;

type DECLARATIONS = HashMap<&'static str, HashMap<&'static str, &'static str>>;

lazy_static! {
    static ref XHTML_PUBLIC_IDS: [(&'static str, &'static str); 8] = [
        ("html", ""),
        ("5", ""),
        ("1.1", "-//W3C//DTD XHTML 1.1//EN"),
        ("strict", "-//W3C//DTD XHTML 1.0 Strict//EN"),
        ("frameset", "-//W3C//DTD XHTML 1.0 Frameset//EN"),
        ("mobile", "-//WAPFORUM//DTD XHTML Mobile 1.2//EN"),
        ("basic", "-//W3C//DTD XHTML Basic 1.1//EN"),
        ("transitional", "-//W3C//DTD XHTML 1.0 Transitional//EN"),
    ];
    static ref XHTML_SYSTEM_IDS: [(&'static str, &'static str); 8] = [
        ("html", ""),
        ("5", ""),
        ("1.1", "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd"),
        (
            "strict",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd"
        ),
        (
            "frameset",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd"
        ),
        (
            "mobile",
            "http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd"
        ),
        (
            "basic",
            "http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd"
        ),
        (
            "transitional",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"
        ),
    ];
    static ref HTML_PUBLIC_IDS: [(&'static str, &'static str); 5] = [
        ("html", ""),
        ("5", ""),
        ("strict", "-//W3C//DTD HTML 4.01//EN"),
        ("frameset", "-//W3C//DTD HTML 4.01 Frameset//EN"),
        ("transitional", "-//W3C//DTD HTML 4.01 Transitional//EN"),
    ];
    static ref HTML_SYSTEM_IDS: [(&'static str, &'static str); 5] = [
        ("html", ""),
        ("5", ""),
        ("strict", "http://www.w3.org/TR/html4/strict.dtd"),
        ("frameset", "http://www.w3.org/TR/html4/frameset.dtd"),
        ("transitional", "http://www.w3.org/TR/html4/loose.dtd"),
    ];
    pub static ref DOC_TYPES: HashMap<&'static str, DECLARATIONS> =
        HashMap::from([
            (
                "xhtml",
                HashMap::from([
                    ("public_id", XHTML_PUBLIC_IDS.iter().cloned().collect()),
                    ("system_id", XHTML_SYSTEM_IDS.iter().cloned().collect()),
                ]),
            ),
            (
                "html",
                HashMap::from([
                    ("public_id", HTML_PUBLIC_IDS.iter().cloned().collect()),
                    ("system_id", HTML_SYSTEM_IDS.iter().cloned().collect()),
                ]),
            ),
        ]);
}

pub struct DTD<'a> {
    spec: &'a str,
    name: &'a str,
}

impl<'a> DTD<'a> {
    // TODO: check arguments
    pub fn new(spec: &'a str, name: &'a str) -> Self {
        DTD { spec, name }
    }

    fn decralation(&self, id: &'a str) -> Option<&'static str> {
        let value = DOC_TYPES
            .get(self.spec)?
            .get(id)?
            .get(self.name)
            .unwrap_or(&"");
        Some(value)
    }

    // returns empty &'static str if invalid spec or name is given.
    pub fn public_id(&self) -> &'static str {
        self.decralation("public_id").unwrap_or("")
    }

    // returns empty &'static str if invalid spec or name is given.
    pub fn system_id(&self) -> &'static str {
        self.decralation("system_id").unwrap_or("")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let dtd = DTD::new("unknown", "");
        assert_eq!(dtd.spec, "unknown");
        assert_eq!(dtd.name, "");

        let dtd = DTD::new("xhtml", "invalid");
        assert_eq!(dtd.spec, "xhtml");
        assert_eq!(dtd.name, "invalid");

        let dtd = DTD::new("xhtml", "frameset");
        assert_eq!(dtd.spec, "xhtml");
        assert_eq!(dtd.name, "frameset");

        let dtd = DTD::new("html", "invalid");
        assert_eq!(dtd.spec, "html");
        assert_eq!(dtd.name, "invalid");

        let dtd = DTD::new("html", "strict");
        assert_eq!(dtd.spec, "html");
        assert_eq!(dtd.name, "strict");
    }

    #[test]
    fn test_public_id() {
        let dtd = DTD::new("invalid", "");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("invalid", "html");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("xhtml", "unknown");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("xhtml", "5");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("xhtml", "1.1");
        assert_eq!(dtd.public_id(), "-//W3C//DTD XHTML 1.1//EN");

        let dtd = DTD::new("html", "unknown");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("html", "5");
        assert_eq!(dtd.public_id(), "");

        let dtd = DTD::new("html", "strict");
        assert_eq!(dtd.public_id(), "-//W3C//DTD HTML 4.01//EN");
    }
}
