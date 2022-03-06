use lithe::parse;

fn main() {
    // TODO: read input file
    parse("/ Hoi Zäme!");
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn test_parse() {
        // TODO: add unit tests
        parse("/ Foo\n");
        parse("/! Bar");

        parse("doctype xml");
        parse("doctype  xml");
        parse("doctype xml ISO-8859-1");
        parse("doctype html");
        parse("doctype 5");
        parse("doctype 1.1");
        parse("doctype strict");
        parse("doctype frameset");
        parse("doctype mobile");
        parse("doctype mobile");
        parse("doctype basic");
        parse("doctype transitional");

        parse("doctype unknown");
    }
}
