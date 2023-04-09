extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "cv.pest"]
pub struct CVParser;

fn get_value(pair: Pair<Rule>) -> Result<&str, String> {
    let mut inner_rules = pair.into_inner();
    let val = inner_rules.next().ok_or("no value")?.as_str();
    Ok(val)
}

fn main() -> Result<(), ()> {
    let unparsed_file = fs::read_to_string("cv.ini")
        .map_err(|err| eprintln!("ERROR: Failed to read file: {err}"))?;

    let file = CVParser::parse(Rule::cv, &unparsed_file)
        .map_err(|err| eprintln!("ERROR: Failed to parse file: {err}"))?
        .next()
        .ok_or_else(|| eprintln!("ERROR: File is empty"))?;

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::about => {
                let section = properties.entry("about").or_default();
                for inner_rule in line.into_inner() {
                    match inner_rule.as_rule() {
                        Rule::email => {
                            let value =
                                get_value(inner_rule).map_err(|err| eprintln!("ERROR: {err}"))?;
                            section.insert("email", value);
                        }
                        Rule::phone => {
                            let value =
                                get_value(inner_rule).map_err(|err| eprintln!("ERROR: {err}"))?;
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

    Ok(())
}
