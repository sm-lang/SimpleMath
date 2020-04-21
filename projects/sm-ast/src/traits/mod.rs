mod tex;
mod wolfram;

pub use wolfram_wxf::ToWolfram;

pub trait ToTex {
    fn to_tex(&self) -> String;
}
