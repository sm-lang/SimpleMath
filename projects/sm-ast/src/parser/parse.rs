use crate::{
    parser::{ParserSettings, CLIMBER},
    ToWolfram, AST,
};
use sm_parser::{Pair, Parser, Rule, SMParser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use num::{BigInt, Num};
use num::bigint::ParseBigIntError;

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserSettings {
    pub fn parse_file(&self, path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        let s = self.parse(&r);
        let mut file = File::create(path_to)?;
        file.write_all(&s.to_wolfram_bytes())?;
        return Ok(());
    }
    pub fn parse(&self, text: &str) -> AST {
        let pairs = SMParser::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::expression => {
                    self.parse_expression(pair);
                }
                Rule::COMMENT => continue,
                _ => debug_cases!(pair),
            };
        }
        //        println!("{:#?}", codes);
        //        unreachable!();
        return AST::Null;
    }

    fn parse_expression(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![""];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => {
                    self.parse_expr(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::WHITESPACE => AST::EmptyStatement,
                Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                Rule::bracket_call => debug_cases!(pair),
                _ => debug_cases!(pair),
            },
            |lhs: AST, op: Pair<Rule>, rhs: AST| match op.as_rule() {
                _ => AST::Binary(Box::from(op.as_str()), Box::new(lhs), Box::new(rhs))
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![""];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::node => {
                    self.parse_node(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![""];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::data => {
                    self.parse_data(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![""];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Symbol => {
                    self.parse_symbol(pair);
                }
                Rule::Boolean => {
                    return match pair.as_str() {
                        "true" => AST::Boolean(true),
                        "false" => AST::Boolean(false),
                        _ => unreachable!()
                    };
                }
                Rule::Byte => {
                    return self.parse_byte(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_symbol(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![""];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => {
                    println!("{}", pair.as_str())
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_byte(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            return match pair.as_rule() {
                Rule::Byte_HEX => {
                    let s = pair.as_str();
                    match BigInt::from_str_radix(&s[2..s.len()], 16) {
                        Ok(o) => AST::Integer(o),
                        Err(_) => panic!(""),
                    }
                }
                Rule::Byte_OCT => {
                    let s = pair.as_str();
                    match BigInt::from_str_radix(&s[2..s.len()], 8) {
                        Ok(o) => AST::Integer(o),
                        Err(_) => panic!(""),
                    }
                }
                Rule::Byte_BIN => {
                    let s = pair.as_str();
                    match BigInt::from_str_radix(&s[2..s.len()], 2) {
                        Ok(o) => AST::Integer(o),
                        Err(_) => panic!(""),
                    }
                }
                _ => unreachable!(),
            };
        }
        return AST::Null;
    }
}
