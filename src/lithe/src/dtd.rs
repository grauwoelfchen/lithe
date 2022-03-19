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

pub struct DTD {
    spec: &'static str,
}

impl DTD {
    pub fn new(spec: &'static str) -> Self {
        DTD { spec }
    }

    pub fn public_id(&self, definition: &str) -> &'static str {
        DOC_TYPES
            .get(self.spec)
            .unwrap()
            .get("public_id")
            .unwrap()
            .get(definition)
            .unwrap_or(&"")
    }

    pub fn system_id(&self, definition: &str) -> &'static str {
        DOC_TYPES
            .get(self.spec)
            .unwrap()
            .get("system_id")
            .unwrap()
            .get(definition)
            .unwrap_or(&"")
    }
}
