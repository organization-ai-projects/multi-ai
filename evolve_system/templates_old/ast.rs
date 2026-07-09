use syn::{File, Item};

pub fn parse_rust_ast(code: &str) -> Vec<String> {
    let syntax: File = syn::parse_file(code).unwrap();
    syntax
        .items
        .iter()
        .map(|item| match item {
            Item::Fn(f) => format!("fn {}({})", f.sig.ident, f.sig.inputs.len()),
            Item::Struct(s) => format!("struct {}", s.ident),
            _ => "other".to_string(),
        })
        .collect()
}
