#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LParens,
    RParens,

    Dot,

    // Keywords
    Ident(String),
    Method,
    // Constant,
    Return,
    // Virtual,
    Move,
    // Class,
    Void,

    Tab,
    NewLine,
}

pub fn tokenize(file: &str) -> Vec<Token> {
    let mut chars = file.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(car) = chars.next() {
        match car {
            '(' => tokens.push(Token::LParens),
            ')' => tokens.push(Token::RParens),

            '.' => tokens.push(Token::Dot),

            '\t' => {
                if matches!(
                    tokens.last().unwrap_or(&Token::NewLine),
                    Token::Tab | Token::NewLine
                ) {
                    tokens.push(Token::Tab);
                }
            }

            '\n' => {
                tokens.push(Token::NewLine);
            }

            ' ' => {}

            '\\' => {
                if matches!(
                    tokens.last().unwrap_or(&Token::NewLine),
                    Token::Tab | Token::NewLine
                ) {
                    assert_eq!(chars.next().unwrap(), 't');
                    tokens.push(Token::Tab);
                }
            }

            'a'..='z' | 'A'..='Z' => {
                let mut s = String::from(car);

                while matches!(chars.peek(), Some('a'..='z' | 'A'..='Z')) {
                    s.push(chars.next().unwrap());
                }

                tokens.push(match s.as_str() {
                    "Method" => Token::Method,
                    "Return" => Token::Return,
                    "Move" => Token::Move,
                    "Void" => Token::Void,
                    // "" => Token::,
                    _ => Token::Ident(s),
                });
            }

            _ => panic!(),
        };
    }

    tokens
}

pub struct ClassDef {
    statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Variable(VariableId);

#[derive(Clone, Debug)]
pub struct Method {
    args: Vec<(ClassId, String)>,
    ret: Option<ClassId>,
    statements: Vec<Statement>,
    name: String,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Method(Method),
    MethodCall(MethodCall),
    VariableDeclaration(ClassId, Variable, Value),
}

#[derive(Clone, Debug)]
pub struct Value(ValueType);

#[derive(Clone, Debug)]
pub enum ValueType {
    Method(MethodCall),
    Variable(VariableId),
    Undefined,
    Null,
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    args: Vec<Value>,
    method_name: String,
    target: Box<Value>,
}

// pub enum Class {
//     ClassId(ClassId),
//     Variable(Value),
//     Class,
// }

#[derive(Clone, Debug)]
pub struct ClassId(String);

#[derive(Clone, Debug)]
pub struct VariableId(String);

pub fn parse_statements(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    depth: usize,
) -> Vec<Statement> {
    let mut statements = Vec::new();

    loop {
        for _ in 0..depth {
            assert_eq!(*tokens.next().unwrap(), Token::Tab, "{depth}");
        }

        match tokens.peek() {
            Some(Token::LParens) => {
                let Value(ValueType::Method(call)) = parse_value(tokens) else {
                    unreachable!()
                };

                statements.push(Statement::MethodCall(call));
            }

            Some(Token::Void) => {
                assert_eq!(depth, 0);
                let token = tokens.next().unwrap();

                let Token::Ident(name) = tokens.next().unwrap() else {
                    unreachable!()
                };

                if Token::Method == **tokens.peek().unwrap() {
                    statements.push(Statement::Method(parse_method_definition(
                        tokens,
                        (*token).clone(),
                        name.clone(),
                        depth,
                    )));
                }
            }

            // either a method call
            Some(Token::Ident(_)) => {
                let Token::Ident(name) = tokens.next().unwrap() else {
                    unreachable!()
                };

                match tokens.next().unwrap() {
                    Token::Dot => {
                        let Token::Ident(method_name) = tokens.next().unwrap() else {
                            unreachable!()
                        };

                        let args = parse_args(tokens);

                        statements.push(Statement::MethodCall(MethodCall {
                            args,
                            method_name: method_name.clone(),
                            target: Box::new(Value(ValueType::Variable(VariableId(name.clone())))),
                        }));
                    }
                    Token::Ident(var_name) => {
                        if Token::Method == **tokens.peek().unwrap() {
                            statements.push(Statement::Method(parse_method_definition(
                                tokens,
                                Token::Ident(name.clone()),
                                var_name.clone(),
                                depth,
                            )));
                        } else {
                            statements.push(Statement::VariableDeclaration(
                                ClassId(name.clone()),
                                Variable(VariableId(name.clone())),
                                parse_value(tokens),
                            ));
                        }
                    }

                    _ => unreachable!(),
                }
            }
            x @ Some(_) => unreachable!("{x:?}"),

            None => {
                return statements;
            }
        }

        if let Some(token) = tokens.next() {
            assert_eq!(*token, Token::NewLine);
        } else {
            return statements;
        }
    }
}

fn parse_method_definition(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    ret: Token,
    name: String,
    depth: usize,
) -> Method {
    assert_eq!(*tokens.next().unwrap(), Token::Method);

    let mut args = Vec::new();

    while **tokens.peek().expect("method def should not end a file") != Token::NewLine {
        let Token::Ident(arg_class) = tokens.next().unwrap() else {
            unreachable!()
        };
        let Token::Ident(arg) = tokens.next().unwrap() else {
            unreachable!()
        };

        args.push((ClassId(arg_class.clone()), arg.clone()));
    }

    assert_eq!(*tokens.next().unwrap(), Token::NewLine);

    let ret = if let Token::Ident(class) = ret {
        Some(ClassId(class.clone()))
    } else {
        None
    };

    Method {
        args,
        ret,
        statements: parse_statements(tokens, depth + 1),
        name,
    }
}

fn parse_value(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Value {
    let val = match tokens.next().unwrap() {
        Token::Ident(ident) => Value(ValueType::Variable(VariableId(ident.clone()))),
        Token::LParens => {
            let val = parse_value(tokens);
            assert_eq!(*tokens.next().unwrap(), Token::RParens);

            val
        }

        _ => unreachable!(),
    };

    if Token::Dot == **tokens.peek().unwrap() {
        assert_eq!(Token::Dot, *tokens.next().unwrap());

        let Token::Ident(method_name) = tokens.next().unwrap() else {
            unreachable!()
        };

        let args = parse_args(tokens);

        Value(ValueType::Method(MethodCall {
            args,
            method_name: method_name.clone(),
            target: Box::new(val),
        }))
    } else {
        val
    }
}

// this goes parens to parens i.e. (...)
fn parse_args(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Vec<Value> {
    assert_eq!(*tokens.next().unwrap(), Token::LParens);

    let mut values = Vec::new();

    while Token::RParens != **tokens.peek().unwrap() {
        values.push(parse_value(tokens));
    }

    assert_eq!(*tokens.next().unwrap(), Token::RParens);

    return values;
}
fn optional_constume(tokens: &mut std::iter::Peekable<std::slice::Iter<&Token>>, token: &Token) {
    if let Some(peek) = tokens.peek() {
        if peek == &&token {
            tokens.next().unwrap();
        }
    }
}
