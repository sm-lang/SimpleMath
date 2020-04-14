pub mod wolfram;
pub mod tex;

pub use wolfram_wxf::ToWolfram;

pub trait ToTex {
    fn to_tex(&self) -> String;
}
