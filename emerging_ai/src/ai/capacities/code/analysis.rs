use syn::{visit::Visit, File, ItemFn};

pub struct CodeAnalysis {
    complexity: u32,
    nesting_depth: u32,
    function_count: u32,
}

impl CodeAnalysis {
    pub fn new() -> Self {
        Self {
            complexity: 0,
            nesting_depth: 0,
            function_count: 0,
        }
    }

    pub fn analyze(&mut self, code: &str) -> f32 {
        let ast = syn::parse_str::<File>(code)
            .unwrap_or_else(|_| syn::parse_str("fn dummy() {}").unwrap());

        self.visit_file(&ast);
        self.calculate_score()
    }

    fn calculate_score(&self) -> f32 {
        let complexity_score = 1.0 / (1.0 + self.complexity as f32 * 0.1);
        let depth_score = 1.0 / (1.0 + self.nesting_depth as f32 * 0.2);
        let function_score = if self.function_count > 0 { 1.0 } else { 0.5 };

        (complexity_score + depth_score + function_score) / 3.0
    }
}

impl<'ast> Visit<'ast> for CodeAnalysis {
    // ...implémentation des méthodes de visite...
}
