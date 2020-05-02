use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

#[test]
#[ignore]
pub fn gen_parser() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./simple-math.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "./src/sm_parser.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct SMParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct SMParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}
