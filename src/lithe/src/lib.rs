#![allow(clippy::upper_case_acronyms)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;
use pest::RuleType;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LitheParser;

#[allow(dead_code)]
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn print_pairs<T>(pairs: &mut Pairs<T>, level: usize)
where
    T: RuleType,
{
    let indent = " ".repeat(level);

    for pair in pairs {
        let rule = pair.as_rule();
        let span = pair.as_span();

        let tag = format!("{:?}", &rule);
        if tag == "EOI" {
            println!();
            continue;
        }

        println!("{}Rule: {:?}", indent, rule);
        println!("{}Span: {:?}", indent, span);

        let mut inner = pair.into_inner();

        // TODO: print inner text at here
        if tag == "comment" {
            if let Some(text) = inner.clone().last() {
                println!("{}Text: {:?}", indent, text);
            }
        }

        print_pairs(&mut inner, level + 2);
    }
}

pub fn parse(s: &str) {
    let mut result = LitheParser::parse(Rule::document, s)
        .unwrap_or_else(|e| panic!("{}", e));

    print_pairs(&mut result, 0);
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

    #[test]
    fn test_doctype() {
        let doctypes = vec![
            "doctype xml",
            "doctype xml ISO-8859-1",
            "doctype html",
            "doctype  5",
            "doctype\n1.1",
            "doctype\n\n\n  strict",
        ];
        for d in doctypes.iter() {
            assert_rule!(Rule::doctype, d);
        }
    }
}
