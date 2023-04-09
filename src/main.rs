extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use std::collections::HashMap;
use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "cv.pest"]
pub struct CVParser;

fn get_value(pair: Pair<Rule>) -> Result<&str, String> {
    let mut inner_rules = pair.into_inner();
    let val = inner_rules.next().ok_or("no value")?.as_str();
    Ok(val)
}

fn main() {
    let unparsed_file = fs::read_to_string("cv.ini")
        .expect("read file");

    let file = CVParser::parse(Rule::cv, &unparsed_file)
        .expect("successful parse")
        .next()
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::about => {
                let section = properties.entry("about").or_default();
                for inner_rule in line.into_inner() {
                    match inner_rule.as_rule() {
                        Rule::email => {
                            let value = get_value(inner_rule).unwrap();
                            section.insert("email", value);
                        }
                        Rule::phone => {
                            let value = get_value(inner_rule).unwrap();
                            section.insert("phone", value);
                        }
                        Rule::EOI => todo!(),
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}
