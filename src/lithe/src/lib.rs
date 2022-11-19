#![allow(clippy::upper_case_acronyms)]

extern crate anyhow;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::RuleType;

mod dtd;
pub mod document;
pub mod document_type;

pub mod parser;

#[allow(dead_code)]
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[allow(dead_code)]
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
