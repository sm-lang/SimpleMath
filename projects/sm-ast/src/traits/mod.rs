#[allow(dead_code)]
mod tex;
mod wolfram;
mod pretty;

pub use wolfram_wxf::ToWolfram;

pub trait ToTex {
    fn to_tex(&self) -> String;
}

pub trait ToPretty {
    fn to_pretty(&self) -> String;
}