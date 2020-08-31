use crate::{
    ast::{Expression, Parameter, Position, Symbol},
    parser::{infix_map, prefix_map, suffix_map, ParserSettings, CLIMBER},
    SMResult, ToWolfram, AST,
};
use bigdecimal::BigDecimal;
use num::{BigInt, Num};
use sm_parser::{Pair, Parser, Rule, SMParser};
use std::{
    collections::BTreeMap,
    fs::{read_to_string, File},
    io::Write,
    iter::FromIterator,
    str::FromStr,
};

impl ParserSettings {
    pub fn parse_file(&self, path_from: &str, path_to: &str) -> SMResult<()> {
        let r = read_to_string(path_from)?;
        let s = self.parse(&r)?;
        let mut file = File::create(path_to)?;
        file.write_all(&s.to_wolfram_bytes())?;
        return Ok(());
    }
    pub fn parse_repl(&self, _text: &str) -> Vec<Expression> {
        unimplemented!()
    }

    pub fn parse(&self, text: &str) -> SMResult<AST> {
        let pairs = SMParser::parse(Rule::program, text)?;
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::expression => {
                    return Ok(self.parse_expression(pair).0);
                }
                Rule::COMMENT => continue,
                _ => debug_cases!(pair),
            };
        }
        return Ok(AST::EmptyStatement);
    }

    fn parse_expression(&self, pairs: Pair<Rule>) -> (AST, bool) {
        let mut eos = false;
        let mut base = AST::EmptyStatement;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::expr => {
                    base = self.parse_expr(pair);
                }
                Rule::eos => eos = true,
                _ => debug_cases!(pair),
            };
        }
        return (base, eos);
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |lhs: AST, op: Pair<Rule>, rhs: AST| match op.as_rule() {
                Rule::Dot => self.parse_dot_call(lhs, rhs, &position),
                _ => {
                    let p = Parameter {
                        arguments: vec![lhs, rhs],
                        options: Default::default(),
                        position: position.clone(),
                    };
                    AST::Function(infix_map(op.as_str()), vec![p])
                }
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        let mut base = AST::EmptyStatement;
        let mut prefix = vec![];
        let mut ps = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::node => {
                    base = self.parse_node(pair);
                }
                Rule::Prefix => {
                    ps.push(self.get_position(pair.as_span()));
                    prefix.push(pair.as_str().to_string())
                }
                Rule::Suffix => {
                    let position = self.get_position(pair.as_span());
                    let p = Parameter { arguments: vec![base], options: Default::default(), position };
                    base = AST::Function(suffix_map(pair.as_str()), vec![p])
                }
                Rule::slice => {
                    let position = self.get_position(pair.as_span());
                    let mut new = vec![base];
                    new.extend_from_slice(&self.parse_slice(pair));
                    let s = Symbol::from("std::core::Index");
                    let p = Parameter { arguments: new, options: Default::default(), position };
                    base = AST::Function(s, vec![p])
                }
                _ => debug_cases!(pair),
            };
        }
        for (a, b) in prefix.iter().rev().zip(ps.iter().rev()) {
            let p = Parameter { arguments: vec![base], options: Default::default(), position: b.clone() };
            base = AST::Function(prefix_map(a), vec![p])
        }
        return base;
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.clone().into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::expr => self.parse_expr(pair),
            Rule::data => self.parse_data(pair),
            Rule::apply_call => self.parse_apply_call(pair),
            Rule::space_call => self.parse_space_call(pair),
            _ => debug_cases!(pair),
        }
    }

    fn parse_dot_call(&self, lhs: AST, rhs: AST, position: &Position) -> AST {
        return match rhs {
            AST::Symbol(rs) => {
                // TODO: dot call resolve
                // let s = Symbol::from("std::infix::dot_call");
                // let p1 = Parameter { arguments: vec![AST::string(rs.name)], options: Default::default(), position: position.clone() };
                let p2 = Parameter { arguments: vec![lhs], options: Default::default(), position: position.clone() };
                AST::Function(rs, vec![p2])
            }
            AST::Function(rs, mut p) => {
                let old = &mut p[0].arguments;
                let mut new = vec![lhs];
                new.extend_from_slice(&old);
                *old = new;
                AST::Function(rs, p)
            }
            AST::Integer(i) => {
                let s = Symbol::from("std::core::index");
                let p = Parameter { arguments: vec![lhs, AST::integer(i)], options: Default::default(), position: position.clone() };
                AST::Function(s, vec![p])
            }
            _ => unreachable!(),
        };
    }

    fn parse_apply_call(&self, pairs: Pair<Rule>) -> AST {
        let mut pairs = pairs.into_inner();
        let head = Symbol::from(pairs.next().unwrap().as_str());
        let mut stack = vec![];
        for pair in pairs {
            match pair.as_rule() {
                // Rule::Symbol => unreachable!(),
                Rule::apply => stack.push(self.parse_apply(pair)),
                _ => debug_cases!(pair),
            };
        }
        return AST::Function(head, stack);
    }

    fn parse_space_call(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        let mut stack = vec![];
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
        let p = Parameter { arguments: stack, options: Default::default(), position };
        return AST::Function(infix_map("*"), vec![p]);
    }

    fn parse_apply(&self, pairs: Pair<Rule>) -> Parameter {
        let position = self.get_position(pairs.as_span());
        let mut args = vec![];
        let mut kws = BTreeMap::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::Comma => continue,
                Rule::apply_kv => {
                    let (k, v) = self.parse_apply_kv(pair);
                    match k {
                        AST::EmptyStatement => args.push(v),
                        _ => {
                            kws.insert(k, v);
                        }
                    }
                }
                _ => unreachable!(),
            };
        }
        return Parameter { arguments: args, options: kws, position };
    }

    fn parse_apply_kv(&self, pairs: Pair<Rule>) -> (AST, AST) {
        let (mut key, mut value) = (AST::EmptyStatement, AST::EmptyStatement);
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

    fn parse_slice(&self, pairs: Pair<Rule>) -> Vec<AST> {
        let mut slices = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Comma => continue,
                Rule::index => slices.push(self.parse_index(pair)),
                _ => unreachable!(),
            };
        }
        return slices;
    }

    fn parse_index(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        let mut start = AST::integer(1);
        let mut end = AST::symbol("std::core::All");
        let mut step = AST::integer(1);
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Colon => continue,
                Rule::Start => {
                    let e = pair.into_inner().nth(0).unwrap();
                    start = self.parse_expr(e)
                }
                Rule::End => {
                    let e = pair.into_inner().nth(0).unwrap();
                    end = self.parse_expr(e)
                }
                Rule::Step => {
                    let e = pair.into_inner().nth(0).unwrap();
                    step = self.parse_expr(e);
                }
                Rule::expr => return self.parse_expr(pair),
                _ => unreachable!(),
            };
        }
        let s = Symbol::from("std::core::Span");
        let p = Parameter { arguments: vec![start, end, step], options: Default::default(), position };
        return AST::Function(s, vec![p]);
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
            Rule::Special => self.parse_special(pair),
            Rule::REPL => self.parse_in_out(pair),
            Rule::Slot => self.parse_slot(pair),
            _ => debug_cases!(pair),
        }
    }

    fn parse_list(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        let mut v = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::Comma => continue,
                Rule::expr => v.push(self.parse_expr(pair)),
                _ => debug_cases!(pair),
            };
        }
        let s = Symbol::from("std::containers::List");
        let p = Parameter { arguments: v, options: Default::default(), position };
        return AST::Function(s, vec![p]);
    }

    fn parse_string(&self, pairs: Pair<Rule>) -> AST {
        let (mut s, mut h) = ("", "");
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => h = pair.as_str(),
                Rule::StringNormal => {
                    let h = pair.as_str();
                    s = &h[1..h.len() - 1]
                }
                Rule::StringBlock => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::Quotation => continue,
                            Rule::StringText => s = inner.as_str(),
                            _ => debug_cases!(inner),
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

    fn parse_in_out(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        let mut head = vec![];
        let mut n = vec![];
        for c in pairs.as_str().chars() {
            if c == '¶' || c == '⁋' { head.push(c) } else { n.push(c) }
        }
        let input = if n.len() == 0 { -BigInt::from(head.len()) } else { BigInt::from_str(&String::from_iter(n)).unwrap() };
        let s = match head[0] {
            '¶' => Symbol::from("std::repl::input"),
            '⁋' => Symbol::from("std::repl::output"),
            _ => unreachable!(),
        };
        let p = Parameter { arguments: vec![AST::integer(input)], options: Default::default(), position };
        AST::Function(s, vec![p])
    }

    fn parse_slot(&self, pairs: Pair<Rule>) -> AST {
        let position = self.get_position(pairs.as_span());
        let mut slot = vec![];
        let s = if pairs.as_str().starts_with("##") { Symbol::from("std::core::slot_sequence") } else { Symbol::from("std::core::slot") };
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => slot.push(AST::string(pair.as_str())),
                Rule::Integer => {
                    let i = BigInt::from_str(pair.as_str()).unwrap();
                    slot.push(AST::integer(i))
                }
                _ => debug_cases!(pair),
            };
        }
        let p = Parameter { arguments: slot, options: Default::default(), position };
        AST::Function(s, vec![p])
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
