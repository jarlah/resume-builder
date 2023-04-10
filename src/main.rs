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

fn get_value(pair: Pair<Rule>) -> Result<&str, ()> {
    let mut inner_rules = pair.into_inner();
    let val = inner_rules
        .next()
        .ok_or("no value")
        .map_err(|err| eprintln!("ERROR: {err}"))?
        .as_str();
    Ok(val)
}

fn read_file(file: &str) -> Result<String, ()> {
    fs::read_to_string(file).map_err(|err| eprintln!("ERROR: Failed to read file: {err}"))
}

fn parse_file(file: &str) -> Result<Pair<Rule>, ()> {
    CVParser::parse(Rule::cv, file)
        .map_err(|err| eprintln!("ERROR: Failed to parse file: {err}"))?
        .next()
        .ok_or_else(|| eprintln!("ERROR: File is empty"))
}

fn parse_about(rule: Pair<Rule>) -> Result<HashMap<&str, String>, ()> {
    let mut about_section: HashMap<&str, String> = HashMap::new();
    for inner_rule in rule.into_inner() {
        match inner_rule.as_rule() {
            Rule::email => {
                about_section.insert("email", get_value(inner_rule)?.to_owned());
            }
            Rule::phone => {
                about_section.insert("phone", get_value(inner_rule)?.to_owned());
            }
            _ => unreachable!(),
        }
    }
    Ok(about_section)
}

fn main() -> Result<(), ()> {
    let unparsed_file = read_file("cv.ini")?;
    let file = parse_file(&unparsed_file)?;
    let mut properties: HashMap<&str, HashMap<&str, String>> = HashMap::new();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::about => {
                properties.insert("about", parse_about(line)?);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    println!("{:#?}", properties);
    Ok(())
}
