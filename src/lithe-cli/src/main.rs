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
        parse("/ Foo");
        parse("/! Bar");
    }
}
