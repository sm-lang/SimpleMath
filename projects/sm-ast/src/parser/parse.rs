use crate::{
    parser::{ParserSettings, CLIMBER},
    ToWolfram, AST,
};
use num::{BigInt, Num};
use sm_parser::{Pair, Parser, Rule, SMParser};
use std::{
    collections::BTreeMap,
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
                    return self.parse_expression(pair);
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
        let mut eos = false;
        let mut base = AST::Null;
        let position = self.get_position(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => {
                    base = self.parse_expr(pair);
                }
                Rule::eos => eos = true,
                _ => debug_cases!(pair),
            };
        }
        return AST::Expression { base: Box::new(base), eos, position };
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        let p = self.get_position(pairs.as_span());
        CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::WHITESPACE => AST::EmptyStatement,
                Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |lhs: AST, op: Pair<Rule>, rhs: AST| match op.as_rule() {
                _ => AST::InfixOperators {
                    infix: op.as_str().to_string(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    position: p.clone(),
                }
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        let mut base = AST::Null;
        let mut prefix = vec![];
        let mut suffix = vec![];
        let position = self.get_position(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::node => {
                    base = self.parse_node(pair);
                }
                Rule::Suffix => suffix.push(pair.as_str().to_string()),
                Rule::Prefix => prefix.push(pair.as_str().to_string()),
                _ => debug_cases!(pair),
            };
        }
        return if prefix.len() + suffix.len() == 0 {
            base
        }
        else {
            AST::UnaryOperators { base: Box::new(base), prefix, suffix, position }
        };
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::data => {
                    return self.parse_data(pair);
                }
                Rule::bracket_call => {
                    return self.parse_bracket_call(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_bracket_call(&self, pairs: Pair<Rule>) -> AST {
        let p = self.get_position(pairs.as_span());
        let mut head = AST::Null;
        let mut args = vec![];
        let mut kws = BTreeMap::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Symbol => {
                    head = AST::symbol(pair.as_str());

                    println!("data: {:?}", head);
                }
                Rule::apply => {
                    (args, kws) = self.parse_apply(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        match head {
            AST::Symbol(s) => {
                return AST::FunctionCall { name: s, arguments: args, options: kws, position: p };
            }
            _ => unreachable!(),
        }
        return AST::Null;
    }

    fn parse_apply(&self, pairs: Pair<Rule>) -> (Vec<AST>, BTreeMap<AST, AST>) {
        let mut args = vec![];
        let mut kws = BTreeMap::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Comma => continue,
                Rule::apply_kv => {
                    let (k, v) = self.parse_apply_kv(pair);
                    match k {
                        AST::Null => args.push(v),
                        _ => kws.insert(k, v).unwrap_none(),
                    }
                }
                _ => unreachable!(),
            };
        }
        return (args, kws);
    }

    fn parse_apply_kv(&self, pairs: Pair<Rule>) -> (AST, AST) {
        let (mut key, mut value) = (AST::Null, AST::Null);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Colon => continue,
                Rule::SYMBOL => key = AST::symbol(pair.as_str()),
                Rule::expr => value = self.parse_expr(pair),
                _ => unreachable!(),
            };
        }
        return (key, value);
    }

    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Symbol => {
                    return AST::symbol(pair.as_str());
                }
                Rule::Boolean => {
                    return match pair.as_str() {
                        "true" => AST::Boolean(true),
                        "false" => AST::Boolean(false),
                        _ => unreachable!(),
                    };
                }
                Rule::Byte => {
                    return self.parse_byte(pair);
                }
                Rule::Integer => {
                    // It is impossible to get `from_str_radix` errors due to the constraints of the parser
                    if let Ok(o) = BigInt::from_str_radix(pair.as_str(), 10) {
                        return AST::Integer(o);
                    }
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::Null;
    }

    fn parse_byte(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            let s = pair.as_str();
            // It is impossible to get `from_str_radix` errors due to the constraints of the parser
            match pair.as_rule() {
                Rule::Byte_HEX => {
                    if let Ok(o) = BigInt::from_str_radix(&s[2..s.len()], 16) {
                        return AST::Integer(o);
                    }
                }
                Rule::Byte_OCT => {
                    if let Ok(o) = BigInt::from_str_radix(&s[2..s.len()], 8) {
                        return AST::Integer(o);
                    }
                }
                Rule::Byte_BIN => {
                    if let Ok(o) = BigInt::from_str_radix(&s[2..s.len()], 2) {
                        return AST::Integer(o);
                    }
                }
                _ => unreachable!(),
            };
        }
        return AST::Null;
    }
}
