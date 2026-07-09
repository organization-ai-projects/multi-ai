#[derive(Debug, Clone)]
pub struct Context {
    pub is_public_api: bool,
    pub is_core: bool,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "is_public_api: {}, is_core: {}",
            self.is_public_api, self.is_core
        )
    }
}
