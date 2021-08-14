#![allow(clippy::upper_case_acronyms)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LitheParser;

pub fn parse(s: &str) {
    // TODO: define more rules
    let result = LitheParser::parse(Rule::comment, s)
        .unwrap_or_else(|e| panic!("{}", e));

    // TODO: neturn something
    for r in result {
        println!("Rule: {:?}", r.as_rule());
        println!("Span: {:?}", r.as_span());

        // code_comment or html_comment
        let c = r.into_inner().next().unwrap();
        let rule = c.as_rule();

        println!("Inner Rule: {:?}", rule);
        println!("Inner Span: {:?}", c.as_span());

        // comment-text
        let t = c.into_inner().last().unwrap();
        println!("Text: {}\n", t.as_str());
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;
    use super::{LitheParser, Rule};

    macro_rules! assert_rule {
        ($rule:expr, $input:expr) => {
            let result = LitheParser::parse($rule, $input)
                .unwrap()
                .peek()
                .unwrap()
                .as_rule();
            assert_eq!(result, $rule);
        };
    }

    #[test]
    fn test_code_comment() {
        let comments = vec![
            "/ foo bar baz qux quux",
            "/foo bar baz qux quux",
            "/  foo bar baz qux quux",
        ];
        for c in comments.iter() {
            assert_rule!(Rule::code_comment, c);
        }
    }

    #[test]
    fn test_html_comment() {
        let comments = vec![
            "/! foo bar baz qux quux",
            "/!foo bar baz qux quux",
            "/!  foo bar baz qux quux",
        ];
        for c in comments.iter() {
            assert_rule!(Rule::html_comment, c);
        }
    }
}
