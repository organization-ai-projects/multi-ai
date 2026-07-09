use crate::token::Token;

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for word in source.split_whitespace() {
        let token = match word {
            "function" => Token::Function,
            "on_event" => Token::OnEvent,
            "run" => Token::Run,
            "let" => Token::Let,
            "if" => Token::If,
            "return" => Token::Return,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            ":" => Token::Colon,
            "," => Token::Comma,
            "(" => Token::ParenOpen,
            ")" => Token::ParenClose,
            "\n" => Token::Newline,
            _ if word.starts_with('"') && word.ends_with('"') => {
                Token::StringLiteral(word.trim_matches('"').to_string())
            }
            _ if matches!(word.parse::<f32>(), Ok(_)) => {
                Token::Number(word.parse::<f32>().unwrap())
            }
            _ => Token::Identifier(word.to_string()),
        };

        tokens.push(token);
    }

    tokens
}
