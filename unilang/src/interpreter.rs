use crate::ast::*;

pub fn interpret(functions: &[Function], entry: &str) {
    if let Some(main) = functions.iter().find(|f| f.name == entry) {
        println!("--- Executing `{}` ---", main.name);
        for stmt in &main.body {
            match stmt {
                Statement::Component(comp) => {
                    println!("Component: {}", comp.name);
                    for (k, v) in &comp.properties {
                        match v {
                            Expr::String(s) => println!("  {} = {}", k, s),
                            _ => {}
                        }
                    }
                }
                Statement::Let { name, value } => {
                    println!("Let {} = {:?}", name, value);
                }
                Statement::Call { name, .. } => {
                    println!("Call: {}", name);
                }
                _ => {}
            }
        }
    } else {
        println!("Function `{}` not found.", entry);
    }
}
