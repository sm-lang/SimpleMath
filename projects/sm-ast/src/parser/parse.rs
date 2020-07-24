use crate::{
    ast::Position,
    parser::{ApplyOrSlice, ParserSettings, CLIMBER},
    SMResult, ToWolfram, AST,
};
use bigdecimal::BigDecimal;
use num::{BigInt, Num};
use sm_parser::{Pair, Parser, Rule, SMParser};
use std::{
    collections::BTreeMap,
    fs::{read_to_string, File},
    io::Write,
};
use crate::ast::{Parameter,Symbol};


macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserSettings {
    pub fn parse_file(&self, path_from: &str, path_to: &str) -> SMResult<()> {
        let r = read_to_string(path_from)?;
        let s = self.parse(&r)?;
        let mut file = File::create(path_to)?;
        file.write_all(&s.to_wolfram_bytes())?;
        return Ok(());
    }
    pub fn parse(&self, text: &str) -> SMResult<AST> {
        let pairs = SMParser::parse(Rule::program, text)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::expression => {
                    return Ok(self.parse_expression(pair));
                }
                Rule::COMMENT => continue,
                _ => debug_cases!(pair),
            };
        }
        return Ok(AST::Null);
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
                Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |lhs: AST, op: Pair<Rule>, rhs: AST| match op.as_rule() {
                Rule::Dot => self.parse_dot_call(lhs, rhs, p.clone()),
                _ => {
                    AST::InfixOperators {
                        infix: op.as_str().to_string(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        position: p.clone(),
                    }
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
        return if prefix.len() + suffix.len() == 0 { base } else { AST::UnaryOperators { base: Box::new(base), prefix, suffix, position } };
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => {
                    return self.parse_expr(pair);
                }
                Rule::data => {
                    return self.parse_data(pair);
                }
                Rule::bracket_call => {
                    return self.parse_bracket_call(pair);
                }
                Rule::space_call => {
                    return self.parse_space_call(pair);
                }
                _ => debug_cases!(pair),
            };
        }
        return AST::EmptyStatement;
    }

    fn parse_dot_call(&self, lhs: AST, rhs: AST, position: Position) -> AST {
        return match rhs {
            /*
            AST::Function { name, arguments, options, .. } => {
                let mut args = vec![lhs];
                args.extend(arguments);
                AST::Function { name, arguments: args, options, position }
            }
            AST::Symbol(s) => AST::Function { name: Box::new(AST::Symbol(s)), arguments: vec![lhs], options: Default::default(), position },
            */
            AST::Integer(_) => unimplemented!(),
            _ => unreachable!(),
        };
    }

    fn parse_bracket_call(&self, pairs: Pair<Rule>) -> AST {
        let mut head = AST::Null;
        let mut parts = vec![];
        let mut stack = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Symbol => {
                    head = AST::symbol(pair.as_str());
                }
                Rule::apply => {
                    parts.push(self.get_position(pair.as_span()));
                    stack.push(self.parse_apply(pair))
                }
                _ => debug_cases!(pair),
            };
        }
        for s in stack {
            let position = parts.pop().unwrap();
            match s {

                ApplyOrSlice::Apply(args, kws) => {
                    unimplemented!()
                    // head = AST::Function { name: Box::new(head), arguments: args, options: kws, position },
                }
                ApplyOrSlice::Slice => unimplemented!()
            }
        }
        return head;
    }

    fn parse_space_call(&self, pairs: Pair<Rule>) -> AST {
        let mut stack = vec![];
        let position = self.get_position(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::Symbol => {
                    stack.push(AST::symbol(pair.as_str()));
                }
                Rule::Integer => {
                    stack.push(self.parse_integer(pair));
                }
                _ => debug_cases!(pair),
            };
        }
        let s = Symbol::from("std::times");
        let p = Parameter {            arguments: stack,            options: Default::default(),            position        };
        return AST::Function(s,vec![p]);
    }

    fn parse_apply(&self, pairs: Pair<Rule>) -> ApplyOrSlice {
        let mut args = vec![];
        let mut kws = BTreeMap::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
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
        return ApplyOrSlice::Apply(args, kws);
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
        let pair = pairs.clone().into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::list => self.parse_list(pair),
            Rule::Symbol => AST::symbol(pair.as_str()),
            Rule::String => self.parse_string(pair),
            Rule::Decimal => self.parse_decimal(pair),
            Rule::Integer => self.parse_integer(pair),
            Rule::Byte => self.parse_byte(pair),
            Rule::SpecialValue => self.parse_special(pair),
            Rule::Input => self.parse_input(pair),
            Rule::Output => self.parse_output(pair),
            _ => debug_cases!(pair),
        }
    }

    fn parse_list(&self, pairs: Pair<Rule>) -> AST {
        let mut v = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Comma => continue,
                Rule::expr => v.push(self.parse_expr(pair)),
                _ => debug_cases!(pair),
            };
        }
        return AST::List(v);
    }

    fn parse_string(&self, pairs: Pair<Rule>) -> AST {
        let mut s = "";
        let mut h = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::StringEmpty => continue,
                Rule::SYMBOL => h = pair.as_str(),
                Rule::StringNormal => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::StringStart => continue,
                            Rule::StringText => s = inner.as_str(),
                            Rule::StringEnd => continue,
                            _ => unreachable!(),
                        }
                    }
                }
                _ => debug_cases!(pair),
            };
        }
        match h {
            "" => AST::string(s),
            _ => AST::string(s),
        }
    }

    fn parse_input(&self, pairs: Pair<Rule>) -> AST {
        let mut head = vec![];
        let mut n = vec![];
        for c in pairs.as_str().chars() {
            if c == '¶' { head.push(c) } else { n.push(c) }
        }
        if n.len() == 0 { AST::string(format!("¶()")) } else { AST::string(format!("¶()")) }
    }
    fn parse_output(&self, pairs: Pair<Rule>) -> AST {
        let mut head = vec![];
        let mut n = vec![];
        for c in pairs.as_str().chars() {
            if c == '⁋' { head.push(c) } else { n.push(c) }
        }
        if n.len() == 0 { AST::string(format!("⁋()")) } else { AST::string(format!("⁋()")) }
    }

    fn parse_byte(&self, pairs: Pair<Rule>) -> AST {
        let s = pairs.as_str();
        let base = match s.chars().nth(1).unwrap() {
            'x' | 'X' => 16,
            'o' | 'O' => 8,
            'b' | 'B' => 2,
            _ => unreachable!(),
        };
        // It is impossible to get `from_str_radix` errors due to the constraints of the parser
        let i = BigInt::from_str_radix(&s[2..s.len()], base).unwrap();
        return AST::Integer(i);
    }

    fn parse_integer(&self, pairs: Pair<Rule>) -> AST {
        // It is impossible to get `from_str_radix` errors due to the constraints of the parser
        let i = BigInt::from_str_radix(pairs.as_str(), 10).unwrap();
        return AST::Integer(i);
    }

    fn parse_decimal(&self, pairs: Pair<Rule>) -> AST {
        // It is impossible to get `from_str_radix` errors due to the constraints of the parser
        let i = BigDecimal::from_str_radix(pairs.as_str(), 10).unwrap();
        return AST::Decimal(i);
    }

    fn parse_special(&self, pairs: Pair<Rule>) -> AST {
        match pairs.as_str() {
            "null" => AST::symbol("std::constant:Null"),
            "true" => AST::Boolean(true),
            "false" => AST::Boolean(false),
            _ => unreachable!(),
        }
    }
}
