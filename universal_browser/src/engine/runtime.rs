use unilang::{ast::Function, lexer, parser};

#[derive(Clone)]
pub struct Page {
    pub name: String,
    pub function: usize,
}

#[derive(Clone)]
pub struct Runtime {
    pub all_functions: Vec<Function>,
    pub current_view: Option<Page>,
}

impl Runtime {
    pub fn new(source: &str) -> Self {
        let tokens = lexer::lex(source);
        let functions = parser::parse(&tokens);

        println!("Fonctions disponibles ({}):", functions.len());
        for f in &functions {
            println!("  - {}", f.name);
        }

        let mut runtime = Self {
            all_functions: functions,
            current_view: None,
        };

        runtime.set_view("Home");
        runtime
    }

    pub fn set_view(&mut self, name: &str) {
        let fname = format!("view_{}", name);
        println!("Recherche de la vue: {}", fname);

        // Recherche plus souple de la vue (ignorer les parenthèses)
        if let Some((index, func)) = self
            .all_functions
            .iter()
            .enumerate()
            .find(|(_, f)| f.name.starts_with(&fname))
        {
            println!("✅ Vue '{}' trouvée (nom exact: {})", fname, func.name);
            self.current_view = Some(Page {
                name: name.to_string(),
                function: index,
            });
        } else {
            println!("❌ Vue '{}' non trouvée", fname);
            // Fallback à la première vue disponible s'il y en a une
            if !self.all_functions.is_empty() {
                let first_view = &self.all_functions[0];
                let view_name = if first_view.name.starts_with("view_") {
                    first_view
                        .name
                        .trim_start_matches("view_")
                        .trim_end_matches("()")
                        .trim_end_matches('(')
                        .trim_end_matches(')')
                        .to_string()
                } else {
                    first_view.name.clone()
                };

                println!(
                    "⚠️ Utilisation de la première vue disponible: {} -> {}",
                    first_view.name, view_name
                );

                self.current_view = Some(Page {
                    name: view_name,
                    function: 0,
                });
            }
        }
    }

    pub fn current_components(&self) -> Vec<unilang::ast::Component> {
        if let Some(page) = &self.current_view {
            if let Some(function) = self.all_functions.get(page.function) {
                let components = function
                    .body
                    .iter()
                    .filter_map(|stmt| {
                        if let unilang::ast::Statement::Component(c) = stmt {
                            Some(unilang::ast::Component {
                                name: c.name.clone(),
                                properties: c.properties.clone(),
                                events: c.events.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                return components;
            }
        }
        Vec::new()
    }

    pub fn current_page_name(&self) -> String {
        self.current_view
            .as_ref()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn load(&mut self, functions: Vec<Function>) {
        self.all_functions = functions;
    }
}
