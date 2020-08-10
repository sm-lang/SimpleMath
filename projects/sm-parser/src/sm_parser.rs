pub struct SMParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    emptyStatement,
    eos,
    comma_or_semi,
    block_or_stmt,
    importStatement,
    use_alias,
    use_module_select,
    module_block,
    module_tuple,
    ModuleSplit,
    As,
    controlFlow,
    block,
    if_statement,
    for_statement,
    for_in_loop,
    re_control,
    Return,
    Yield,
    Break,
    Pass,
    Type,
    classStatement,
    traitStatement,
    short_block,
    short_statement,
    short_annotation,
    extendStatement,
    with_trait,
    assignStatement,
    assign_terms,
    assign_name,
    assign_pair,
    defineStatement,
    define_terms,
    define_parameter,
    define_pair,
    annotation,
    annotation_call,
    apply,
    apply_kv,
    function_name,
    function_module,
    expression,
    expr,
    term,
    node,
    apply_call,
    derivative,
    space_call,
    type_expr,
    type_hint,
    TypeInfix,
    data,
    dict,
    list,
    slice,
    key_valid,
    key_value,
    index,
    Start,
    End,
    Step,
    Number,
    Decimal,
    Integer,
    Byte,
    Positive,
    String,
    StringNormal,
    StringEmpty,
    StringText,
    StringStart,
    StringEnd,
    Symbol,
    SYMBOL,
    SpecialValue,
    Slot,
    REPL,
    WHITESPACE,
    COMMENT,
    MultiLineComment,
    Prefix,
    Suffix,
    Infix,
    Set,
    Derivative,
    Or,
    LazyOr,
    Star,
    Slash,
    Solidus,
    Proportion,
    Comma,
    Dot,
    Separate,
    Semicolon,
    Colon,
    Question,
    Underline,
    Load,
    Save,
    LeftShift,
    RightShift,
    LessEqual,
    GraterEqual,
    Less,
    Grater,
    Pipeline,
    Equivalent,
    NotEquivalent,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Multiply,
    CenterDot,
    Kronecker,
    TensorProduct,
    Divide,
    Quotient,
    Modulo,
    Remainder,
    Power,
    Surd,
    Increase,
    Decrease,
    To,
    Elvis,
    Map,
    Quote,
    Acute,
    Apostrophe,
    Quotation,
    LogicOr,
    LogicAnd,
    LogicNot,
    Ellipsis,
    LogicXor,
    MapAll,
    Concat,
    Destruct,
    DoubleBang,
    Bang,
    Not,
    Sharp,
    Curry,
    Apply,
    At,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for SMParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::emptyStatement(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::importStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::classStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::traitStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::extendStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::controlFlow(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::assignStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::defineStatement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::annotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| self::expression(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn emptyStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::emptyStatement, |state| self::eos(state).or_else(|state| self::Separate(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma_or_semi(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Comma(state).or_else(|state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block_or_stmt(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::block(state)).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::Set(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn importStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::importStatement, |state| state.sequence(|state| state.match_string("using").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::Dot(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::Dot(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_alias(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_alias, |state| state.restore_on_err(|state| state.sequence(|state| self::String(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::As(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))).or_else(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::As(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn use_module_select(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::use_module_select, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.sequence(|state| self::ModuleSplit(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| self::module_block(state)).or_else(|state| self::Star(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| self::comma_or_semi(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::module_tuple(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::comma_or_semi(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn module_tuple(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::module_tuple, |state| state.restore_on_err(|state| self::use_alias(state)).or_else(|state| state.restore_on_err(|state| self::use_module_select(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ModuleSplit(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Dot(state).or_else(|state| self::Proportion(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn As(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::As, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("as")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn controlFlow(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.restore_on_err(|state| self::re_control(state)).or_else(|state| state.restore_on_err(|state| self::if_statement(state))).or_else(|state| state.restore_on_err(|state| self::for_statement(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| self::statement(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::statement(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn if_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::if_statement, |state| state.sequence(|state| state.match_string("if").and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::expr(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("then").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("else").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("otherwise").and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state)))))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state)))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_statement, |state| state.sequence(|state| state.match_string("for").and_then(|state| super::hidden::skip(state)).and_then(|state| self::for_in_loop(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn for_in_loop(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::for_in_loop, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("in")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn re_control(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::re_control, |state| state.restore_on_err(|state| state.sequence(|state| self::Return(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| self::Break(state)).or_else(|state| self::Pass(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Return(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Return, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("return")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Yield(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Yield, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("yield")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Break(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Break, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("break")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pass(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Pass, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("pass")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Type, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("type")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn classStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::classStatement, |state| state.sequence(|state| state.match_string("class").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::short_block(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn traitStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::traitStatement, |state| state.sequence(|state| state.match_string("trait").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::short_block(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_block(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_block, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::short_statement(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::short_statement(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_statement, |state| self::emptyStatement(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| state.match_string("def")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.optional(|state| state.match_string("let")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_terms(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))).or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::short_annotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_annotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_annotation, |state| state.sequence(|state| state.sequence(|state| self::annotation_call(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::annotation_call(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::annotation_call(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::short_statement(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn extendStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::extendStatement, |state| state.sequence(|state| state.match_string("extends").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::with_trait(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::short_block(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn with_trait(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::with_trait, |state| state.sequence(|state| state.match_string("with").and_then(|state| super::hidden::skip(state)).and_then(|state| self::Symbol(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assignStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assignStatement, |state| state.sequence(|state| state.match_string("let").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_terms(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_terms(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_name(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::type_hint(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state))))).or_else(|state| state.sequence(|state| self::assign_name(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::type_hint(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::assign_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::assign_pair(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn assign_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::assign_pair, |state| self::Symbol(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn defineStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::defineStatement, |state| state.sequence(|state| state.match_string("def").and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_terms(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_terms(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::assign_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_parameter(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::type_hint(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::block_or_stmt(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_parameter(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_parameter, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::define_pair(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::define_pair(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.match_string(")")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn define_pair(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::define_pair, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::type_hint(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Set(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation, |state| state.sequence(|state| state.sequence(|state| self::annotation_call(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::annotation_call(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| self::annotation_call(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::statement(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn annotation_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::annotation_call, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::At(state).and_then(|state| state.restore_on_err(|state| self::list(state)).or_else(|state| state.restore_on_err(|state| self::apply(state))).or_else(|state| self::Symbol(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::apply, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::apply_kv(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_kv(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::apply_kv, |state| state.restore_on_err(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_name, |state| self::SYMBOL(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_module(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_module, |state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::expr, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::term, |state| state.sequence(|state| state.repeat(|state| self::Prefix(state)).and_then(|state| self::node(state)).and_then(|state| state.repeat(|state| self::Suffix(state).or_else(|state| state.restore_on_err(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::slice(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn node(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::node, |state| state.restore_on_err(|state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")")))).or_else(|state| state.restore_on_err(|state| self::apply_call(state))).or_else(|state| state.restore_on_err(|state| self::derivative(state))).or_else(|state| state.restore_on_err(|state| self::space_call(state))).or_else(|state| state.restore_on_err(|state| self::data(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::apply_call, |state| state.sequence(|state| self::Symbol(state).and_then(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply(state)))).and_then(|state| state.repeat(|state| state.restore_on_err(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn derivative(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::derivative, |state| state.sequence(|state| self::Symbol(state).and_then(|state| self::Derivative(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state))))).and_then(|state| self::apply(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn space_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::space_call, |state| state.sequence(|state| state.restore_on_err(|state| self::apply_call(state)).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state)).and_then(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| state.restore_on_err(|state| self::apply_call(state)).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state))))).and_then(|state| state.repeat(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| state.restore_on_err(|state| self::apply_call(state)).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::TypeInfix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::TypeInfix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_hint(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::type_hint, |state| state.sequence(|state| self::Colon(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::type_expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TypeInfix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TypeInfix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Or(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::data, |state| state.restore_on_err(|state| self::dict(state)).or_else(|state| state.restore_on_err(|state| self::list(state))).or_else(|state| self::SpecialValue(state)).or_else(|state| self::REPL(state)).or_else(|state| self::Slot(state)).or_else(|state| self::Number(state)).or_else(|state| state.restore_on_err(|state| self::String(state))).or_else(|state| self::Symbol(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::dict, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::key_value(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}")))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::list, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]").or_else(|state| state.restore_on_err(|state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::slice, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.restore_on_err(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state))))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_valid(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::key_valid, |state| self::Integer(state).or_else(|state| self::SYMBOL(state)).or_else(|state| state.restore_on_err(|state| self::String(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::key_value, |state| state.sequence(|state| self::key_valid(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::index, |state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::Start(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::End(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::Step(state))))).or_else(|state| state.sequence(|state| state.optional(|state| state.restore_on_err(|state| self::Start(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| state.restore_on_err(|state| self::End(state)))))).or_else(|state| state.restore_on_err(|state| self::expr(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Start(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Start, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn End(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::End, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Step(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Step, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Byte(state).or_else(|state| self::Decimal(state)).or_else(|state| self::Integer(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| self::Integer(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("0").or_else(|state| self::Positive(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Byte, |state| state.sequence(|state| state.match_insensitive("0b").and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_BIN_DIGIT(state)))))).or_else(|state| state.sequence(|state| state.match_insensitive("0o").and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_OCT_DIGIT(state))))))).or_else(|state| state.sequence(|state| state.match_insensitive("0x").and_then(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| self::Underline(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Positive(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Positive, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.optional(|state| self::SYMBOL(state)).and_then(|state| state.restore_on_err(|state| self::StringNormal(state)).or_else(|state| self::StringEmpty(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringNormal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringNormal, |state| state.sequence(|state| self::StringStart(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringText(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::StringEnd(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEmpty(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEmpty, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Quotation(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringText(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringText, |state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::Quotation(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::PEEK(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::ANY(state))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringStart(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringStart, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::Quotation(state).and_then(|state| state.stack_push(|state| state.repeat(|state| self::Quotation(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn StringEnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::StringEnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::POP(state).and_then(|state| self::Quotation(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Symbol(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Symbol, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| state.repeat(|state| state.sequence(|state| self::Proportion(state).and_then(|state| self::SYMBOL(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))).or_else(|state| state.sequence(|state| self::Underline(state).and_then(|state| self::XID_CONTINUE(state)).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SpecialValue(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SpecialValue, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("null").or_else(|state| state.match_string("true")).or_else(|state| state.match_string("false"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Slot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Slot, |state| state.sequence(|state| state.match_string("##").and_then(|state| state.optional(|state| self::Positive(state)))).or_else(|state| state.sequence(|state| state.match_string("#").and_then(|state| state.optional(|state| self::SYMBOL(state).or_else(|state| self::Integer(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn REPL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::REPL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("¶").and_then(|state| self::Positive(state))).or_else(|state| state.sequence(|state| state.match_string("⁋").and_then(|state| self::Positive(state)))).or_else(|state| state.sequence(|state| state.match_string("¶").and_then(|state| state.repeat(|state| state.match_string("¶"))))).or_else(|state| state.sequence(|state| state.match_string("⁋").and_then(|state| state.repeat(|state| state.match_string("⁋")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.match_string("///").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))).or_else(|state| state.sequence(|state| state.match_string("//?").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))).or_else(|state| state.sequence(|state| state.match_string("//!").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))).or_else(|state| state.sequence(|state| state.match_string("//*").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MultiLineComment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::MultiLineComment, |state| state.sequence(|state| state.match_string("%%%").and_then(|state| state.repeat(|state| self::MultiLineComment(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("%%%")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("%%%")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Prefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Prefix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Not(state)).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Star(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Suffix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Suffix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Derivative(state).or_else(|state| self::DoubleBang(state)).or_else(|state| self::Bang(state)).or_else(|state| self::Question(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Set(state).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Multiply(state)).or_else(|state| self::CenterDot(state)).or_else(|state| self::Kronecker(state)).or_else(|state| self::TensorProduct(state)).or_else(|state| self::Divide(state)).or_else(|state| self::Quotient(state)).or_else(|state| self::Modulo(state)).or_else(|state| self::Power(state)).or_else(|state| self::Grater(state)).or_else(|state| self::GraterEqual(state)).or_else(|state| self::Equal(state)).or_else(|state| self::Dot(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Derivative(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Derivative, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("'").and_then(|state| state.repeat(|state| state.match_string("'"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Or(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Or, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LazyOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LazyOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Star(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Star, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Slash(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Slash, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("/")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Solidus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Solidus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Proportion(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Proportion, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("::").or_else(|state| state.match_string("∷"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",").or_else(|state| state.match_string("，"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Separate(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Separate, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";;")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Semicolon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Semicolon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";").or_else(|state| state.match_string("；"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":").or_else(|state| state.match_string("："))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Question(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Question, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Load(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Load, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<<").or_else(|state| state.match_string("⋘"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Save(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Save, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>>").or_else(|state| state.match_string("⋙"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LeftShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LeftShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<").or_else(|state| state.match_string("≪"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RightShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RightShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>").or_else(|state| state.match_string("≫"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LessEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GraterEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GraterEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Less(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Less, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Grater(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Grater, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pipeline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Pipeline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|>")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("===")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEquivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEquivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=!=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equal, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("==")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!=").or_else(|state| state.match_string("≠"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Plus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Plus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Minus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Minus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Multiply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Multiply, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Star(state).or_else(|state| state.match_string("×"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CenterDot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CenterDot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⋅")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Kronecker(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Kronecker, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊗")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TensorProduct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TensorProduct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊙")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Divide(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Divide, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Slash(state).or_else(|state| state.match_string("÷"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotient(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotient, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("//")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Modulo(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Modulo, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("%")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Remainder(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Remainder, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⁒")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Power(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Power, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Surd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Surd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("√")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Increase(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Increase, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("++")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decrease(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Decrease, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("--")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn To(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::To, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("->")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Elvis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Elvis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Map(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Map, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("/@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quote, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Acute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Acute, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("´")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apostrophe(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apostrophe, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("'")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotation(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotation, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\"")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||").or_else(|state| state.match_string("∧"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicAnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicAnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("&&").or_else(|state| state.match_string("∨"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicNot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicNot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Ellipsis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Ellipsis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("...").or_else(|state| state.match_string("…"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicXor(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicXor, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊕")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MapAll(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MapAll, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("//@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Concat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Concat, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~~")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Destruct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Destruct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DoubleBang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::DoubleBang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Bang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Bang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Not(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Not, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Curry(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Curry, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@@@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apply, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn At(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::At, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_peek()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.stack_pop()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::WHITE_SPACE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::emptyStatement => rules::emptyStatement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::block_or_stmt => rules::block_or_stmt(state),
            Rule::importStatement => rules::importStatement(state),
            Rule::use_alias => rules::use_alias(state),
            Rule::use_module_select => rules::use_module_select(state),
            Rule::module_block => rules::module_block(state),
            Rule::module_tuple => rules::module_tuple(state),
            Rule::ModuleSplit => rules::ModuleSplit(state),
            Rule::As => rules::As(state),
            Rule::controlFlow => rules::controlFlow(state),
            Rule::block => rules::block(state),
            Rule::if_statement => rules::if_statement(state),
            Rule::for_statement => rules::for_statement(state),
            Rule::for_in_loop => rules::for_in_loop(state),
            Rule::re_control => rules::re_control(state),
            Rule::Return => rules::Return(state),
            Rule::Yield => rules::Yield(state),
            Rule::Break => rules::Break(state),
            Rule::Pass => rules::Pass(state),
            Rule::Type => rules::Type(state),
            Rule::classStatement => rules::classStatement(state),
            Rule::traitStatement => rules::traitStatement(state),
            Rule::short_block => rules::short_block(state),
            Rule::short_statement => rules::short_statement(state),
            Rule::short_annotation => rules::short_annotation(state),
            Rule::extendStatement => rules::extendStatement(state),
            Rule::with_trait => rules::with_trait(state),
            Rule::assignStatement => rules::assignStatement(state),
            Rule::assign_terms => rules::assign_terms(state),
            Rule::assign_name => rules::assign_name(state),
            Rule::assign_pair => rules::assign_pair(state),
            Rule::defineStatement => rules::defineStatement(state),
            Rule::define_terms => rules::define_terms(state),
            Rule::define_parameter => rules::define_parameter(state),
            Rule::define_pair => rules::define_pair(state),
            Rule::annotation => rules::annotation(state),
            Rule::annotation_call => rules::annotation_call(state),
            Rule::apply => rules::apply(state),
            Rule::apply_kv => rules::apply_kv(state),
            Rule::function_name => rules::function_name(state),
            Rule::function_module => rules::function_module(state),
            Rule::expression => rules::expression(state),
            Rule::expr => rules::expr(state),
            Rule::term => rules::term(state),
            Rule::node => rules::node(state),
            Rule::apply_call => rules::apply_call(state),
            Rule::derivative => rules::derivative(state),
            Rule::space_call => rules::space_call(state),
            Rule::type_expr => rules::type_expr(state),
            Rule::type_hint => rules::type_hint(state),
            Rule::TypeInfix => rules::TypeInfix(state),
            Rule::data => rules::data(state),
            Rule::dict => rules::dict(state),
            Rule::list => rules::list(state),
            Rule::slice => rules::slice(state),
            Rule::key_valid => rules::key_valid(state),
            Rule::key_value => rules::key_value(state),
            Rule::index => rules::index(state),
            Rule::Start => rules::Start(state),
            Rule::End => rules::End(state),
            Rule::Step => rules::Step(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::Integer => rules::Integer(state),
            Rule::Byte => rules::Byte(state),
            Rule::Positive => rules::Positive(state),
            Rule::String => rules::String(state),
            Rule::StringNormal => rules::StringNormal(state),
            Rule::StringEmpty => rules::StringEmpty(state),
            Rule::StringText => rules::StringText(state),
            Rule::StringStart => rules::StringStart(state),
            Rule::StringEnd => rules::StringEnd(state),
            Rule::Symbol => rules::Symbol(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::SpecialValue => rules::SpecialValue(state),
            Rule::Slot => rules::Slot(state),
            Rule::REPL => rules::REPL(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::MultiLineComment => rules::MultiLineComment(state),
            Rule::Prefix => rules::Prefix(state),
            Rule::Suffix => rules::Suffix(state),
            Rule::Infix => rules::Infix(state),
            Rule::Set => rules::Set(state),
            Rule::Derivative => rules::Derivative(state),
            Rule::Or => rules::Or(state),
            Rule::LazyOr => rules::LazyOr(state),
            Rule::Star => rules::Star(state),
            Rule::Slash => rules::Slash(state),
            Rule::Solidus => rules::Solidus(state),
            Rule::Proportion => rules::Proportion(state),
            Rule::Comma => rules::Comma(state),
            Rule::Dot => rules::Dot(state),
            Rule::Separate => rules::Separate(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Colon => rules::Colon(state),
            Rule::Question => rules::Question(state),
            Rule::Underline => rules::Underline(state),
            Rule::Load => rules::Load(state),
            Rule::Save => rules::Save(state),
            Rule::LeftShift => rules::LeftShift(state),
            Rule::RightShift => rules::RightShift(state),
            Rule::LessEqual => rules::LessEqual(state),
            Rule::GraterEqual => rules::GraterEqual(state),
            Rule::Less => rules::Less(state),
            Rule::Grater => rules::Grater(state),
            Rule::Pipeline => rules::Pipeline(state),
            Rule::Equivalent => rules::Equivalent(state),
            Rule::NotEquivalent => rules::NotEquivalent(state),
            Rule::Equal => rules::Equal(state),
            Rule::NotEqual => rules::NotEqual(state),
            Rule::Plus => rules::Plus(state),
            Rule::Minus => rules::Minus(state),
            Rule::Multiply => rules::Multiply(state),
            Rule::CenterDot => rules::CenterDot(state),
            Rule::Kronecker => rules::Kronecker(state),
            Rule::TensorProduct => rules::TensorProduct(state),
            Rule::Divide => rules::Divide(state),
            Rule::Quotient => rules::Quotient(state),
            Rule::Modulo => rules::Modulo(state),
            Rule::Remainder => rules::Remainder(state),
            Rule::Power => rules::Power(state),
            Rule::Surd => rules::Surd(state),
            Rule::Increase => rules::Increase(state),
            Rule::Decrease => rules::Decrease(state),
            Rule::To => rules::To(state),
            Rule::Elvis => rules::Elvis(state),
            Rule::Map => rules::Map(state),
            Rule::Quote => rules::Quote(state),
            Rule::Acute => rules::Acute(state),
            Rule::Apostrophe => rules::Apostrophe(state),
            Rule::Quotation => rules::Quotation(state),
            Rule::LogicOr => rules::LogicOr(state),
            Rule::LogicAnd => rules::LogicAnd(state),
            Rule::LogicNot => rules::LogicNot(state),
            Rule::Ellipsis => rules::Ellipsis(state),
            Rule::LogicXor => rules::LogicXor(state),
            Rule::MapAll => rules::MapAll(state),
            Rule::Concat => rules::Concat(state),
            Rule::Destruct => rules::Destruct(state),
            Rule::DoubleBang => rules::DoubleBang(state),
            Rule::Bang => rules::Bang(state),
            Rule::Not => rules::Not(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::Curry => rules::Curry(state),
            Rule::Apply => rules::Apply(state),
            Rule::At => rules::At(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
