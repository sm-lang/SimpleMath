use crate::parser::Settings;
use sm_parser::{Pair, Parser, Rule, SMParser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r);
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        return Ok(());
    }
    pub fn format(&self, text: &str) -> String {
        let pairs = SMParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => code.push_str(&format!("{}\n", pair.as_str())),
                _ => debug_cases!(pair),
            };
        }
        //        println!("{:#?}", codes);
        //        unreachable!();
        return code;
    }

    fn format_expression(&self, pairs: Pair<Rule>) -> String {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT => {
                    let c = pair.as_str();
                    codes.push(format!("% {}", c[1..c.len()].trim()))
                }
                _ => debug_cases!(pair),
            };
        }
        return codes.join("");
    }

}
