use crate::ast::{Component, Expr, Function, FunctionKind, Statement};
use crate::token::Token;
use crate::token::Token::*;

pub fn parse(tokens: &[Token]) -> Vec<Function> {
    let mut functions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        if let Function = tokens[i] {
            if let Identifier(name) = &tokens[i + 1] {
                let kind = if name.starts_with("view_on_event_") {
                    FunctionKind::ViewOnEvent
                } else if name.starts_with("view_") {
                    FunctionKind::View
                } else {
                    FunctionKind::Logic
                };

                i += 3; // skip function NAME {
                let mut body = Vec::new();
                while i < tokens.len() && tokens[i] != RBrace {
                    match &tokens[i] {
                        Let => {
                            if let Identifier(var) = &tokens[i + 1] {
                                if let Colon = tokens[i + 2] {
                                    if let StringLiteral(val) = &tokens[i + 3] {
                                        body.push(Statement::Let {
                                            name: var.clone(),
                                            value: Expr::String(val.clone()),
                                        });
                                        i += 4;
                                    }
                                }
                            }
                        }
                        Identifier(comp_name) => {
                            if tokens.get(i + 1) == Some(&LBrace) {
                                let mut props = Vec::new();
                                let mut events = Vec::new();
                                i += 2;
                                while i < tokens.len() && tokens[i] != RBrace {
                                    if let Identifier(prop) = &tokens[i] {
                                        if tokens.get(i + 1) == Some(&Colon) {
                                            match &tokens[i + 2] {
                                                StringLiteral(val) => {
                                                    props.push((
                                                        prop.clone(),
                                                        Expr::String(val.clone()),
                                                    ));
                                                    i += 3;
                                                }
                                                Identifier(handler) => {
                                                    if prop == "on_event" {
                                                        if let Some(Identifier(target)) =
                                                            tokens.get(i + 3)
                                                        {
                                                            events.push((
                                                                handler.clone(),
                                                                target.clone(),
                                                            ));
                                                            i += 4;
                                                        } else {
                                                            i += 1;
                                                        }
                                                    } else {
                                                        i += 1;
                                                    }
                                                }
                                                _ => i += 1,
                                            }
                                        } else {
                                            i += 1;
                                        }
                                    } else {
                                        i += 1;
                                    }
                                }
                                body.push(Statement::Component(Component {
                                    name: comp_name.clone(),
                                    properties: props,
                                    events,
                                }));
                                i += 1;
                            }
                        }
                        Run => {
                            if let Identifier(fname) = &tokens[i + 1] {
                                body.push(Statement::Call {
                                    name: fname.clone(),
                                    args: vec![],
                                });
                                i += 2;
                            }
                        }
                        _ => i += 1,
                    }
                }
                i += 1; // skip closing RBrace
                functions.push(Function {
                    name: name.clone(),
                    kind,
                    body,
                });
            }
        } else {
            i += 1;
        }
    }

    functions
}
