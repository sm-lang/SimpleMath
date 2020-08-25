use sm_ast::{parser::ParserSettings, ToTex, ToWolfram};

mod display;
mod tex;
mod wolfram;

pub fn wl_form(s: &str) -> String {
    let parser = ParserSettings::default();
    parser.parse(s).unwrap().to_wolfram_string()
}

pub fn tex_form(s: &str) -> String {
    let parser = ParserSettings::default();
    parser.parse(s).unwrap().to_tex()
}

pub fn display_form(s: &str) -> String {
    let parser = ParserSettings::default();
    format!("{}", parser.parse(s).unwrap())
}
