extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "cv.pest"]
pub struct CVParser;

#[derive(Debug)]
pub struct About {
    pub phone: String,
    pub email: String
}

impl About {
    fn new() -> About {
        About { phone: "".to_owned(), email: "".to_owned() }
    }
}

#[derive(Debug)]
pub struct CVData {
    pub about: About,
}

impl CVData {
    fn new() -> CVData {
        CVData { about: About::new() }
    }
}

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

fn parse_about(rule: Pair<Rule>, about: &mut About) -> Result<(), ()> {
    for inner_rule in rule.into_inner() {
        match inner_rule.as_rule() {
            Rule::email => {
                about.email = get_value(inner_rule)?.to_owned();
            }
            Rule::phone => {
                about.phone = get_value(inner_rule)?.to_owned();
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn main() -> Result<(), ()> {
    let unparsed_file = read_file("cv.ini")?;
    let file = parse_file(&unparsed_file)?;
    let mut cv_data: CVData = CVData::new();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::about => parse_about(line, &mut cv_data.about)?,
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    println!("{:#?}", cv_data);
    Ok(())
}
