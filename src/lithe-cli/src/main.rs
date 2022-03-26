use lithe::parse;

fn main() {
    // TODO: read input file
    let _ = parse("/ Hoi Zäme!");
}

#[cfg(test)]
mod test {
    use super::parse;

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
