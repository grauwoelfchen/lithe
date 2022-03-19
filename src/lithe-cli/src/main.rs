use lithe::parse;

fn main() {
    // TODO: read input file
    let _ = parse("/ Hoi Zäme!");
}

#[cfg(test)]
mod test {
    use super::parse;

    // TODO: test parse results

    #[test]
    fn test_parse() {
        assert!(parse("/ Foo\n").is_ok());
        assert!(parse("/! Bar").is_ok());

        assert!(parse("doctype xml").is_ok());
        assert!(parse("doctype  xml").is_ok());
        assert!(parse("doctype xml ISO-8859-1").is_ok());
        assert!(parse("doctype html").is_ok());
        assert!(parse("doctype 5").is_ok());
        assert!(parse("doctype 1.1").is_ok());
        assert!(parse("doctype strict").is_ok());
        assert!(parse("doctype frameset").is_ok());
        assert!(parse("doctype mobile").is_ok());
        assert!(parse("doctype mobile").is_ok());
        assert!(parse("doctype basic").is_ok());
        assert!(parse("doctype transitional").is_ok());

        assert!(parse("doctype unknown").is_err());
    }
}
