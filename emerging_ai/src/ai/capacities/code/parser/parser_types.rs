#[derive(Debug, Clone)]
pub struct ParseResult {
    pub code: String,
    pub path: String,
    pub ast: Option<syn::File>,
    pub metadata: ParseMetadata,
}

#[derive(Debug, Clone)]
pub struct ParseMetadata {
    pub functions_count: usize,
    pub imports_count: usize,
    pub loc: usize,
}
