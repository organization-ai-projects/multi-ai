#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (nom, type)
    pub return_type: Option<String>,
    pub body: Vec<String>,
}

#[derive(Debug)]
pub enum Statement {
    Return(String),
    Expression(String),
}

impl Function {
    pub fn to_rust(&self) -> String {
        let mut code = format!("pub fn {}(", self.name);
        
        // Params
        let params = self.params.iter()
            .map(|(name, typ)| format!("{}: {}", name, typ))
            .collect::<Vec<_>>()
            .join(", ");
        code.push_str(&params);
        code.push(')');

        // Return type
        if let Some(ret) = &self.return_type {
            code.push_str(&format!(" -> {}", ret));
        }

        // Body
        code.push_str(" {\n");
        for line in &self.body {
            code.push_str(&format!("    {}\n", line));
        }
        code.push_str("}\n");

        code
    }
}

pub fn parse_function(content: &str) -> Option<Function> {
    let mut lines = content.lines();
    let first_line = lines.next()?;

    if !first_line.starts_with("function ") {
        return None;
    }

    // Parse function declaration
    let declaration = first_line.trim_start_matches("function ").trim();
    let name_end = declaration.find('(')?;
    let name = declaration[..name_end].trim().to_string();

    // Parse parameters
    let params_start = declaration.find('(')?;
    let params_end = declaration.find(')')?;
    let params_str = &declaration[params_start + 1..params_end];
    
    let params = params_str.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|param| {
            let parts: Vec<&str> = param.split(':').map(|s| s.trim()).collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    // Parse return type if any
    let return_type = if let Some(arrow_pos) = declaration.find("->") {
        let ret_type = declaration[arrow_pos + 2..].trim()
            .trim_end_matches('{')
            .trim()
            .to_string();
        Some(ret_type)
    } else {
        None
    };

    // Parse body
    let mut body = Vec::new();
    let mut brace_count = 1;

    for line in lines {
        let line = line.trim();
        if line.contains('{') {
            brace_count += 1;
        }
        if line.contains('}') {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        if !line.is_empty() {
            body.push(line.to_string());
        }
    }

    Some(Function {
        name,
        params,
        return_type,
        body,
    })
}
