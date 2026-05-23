use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // literals
    String(String),
    Number(f64),

    LParens,
    RParens,
    // ForwardSlash,
    Dot,

    // Keywords
    Ident(String),
    Method,
    Return,
    // Virtual,
    Move,
    Void,
    Inherit,
    Use,
    As,

    Do,
    Until,
    If,

    Tab,
    NewLine,
}

pub fn tokenize(file: &str) -> Vec<Token> {
    let mut chars = file.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(car) = chars.next() {
        match car {
            '"' => {
                let mut literal = String::new();

                loop {
                    match chars.next().unwrap() {
                        '"' => {
                            break;
                        }
                        '\\' => {
                            assert_eq!('"', chars.next().unwrap());
                            literal.push('"');
                        }

                        car2 => literal.push(car2),
                    }
                }

                tokens.push(Token::String(literal));
            }

            '0'..'9' => {
                let mut literal = String::from(car);

                while let Some('0'..'9') = chars.peek() {
                    literal.push(chars.next().unwrap());
                }

                if let Some('.') = chars.peek() {
                    assert_eq!('.', chars.next().unwrap());
                    literal.push('.');

                    while let Some('0'..'9') = chars.peek() {
                        literal.push(chars.next().unwrap());
                    }
                }

                tokens.push(Token::Number(dbg!(literal).parse().unwrap()));
            }

            '(' => tokens.push(Token::LParens),
            ')' => tokens.push(Token::RParens),
            '.' => tokens.push(Token::Dot),
            // '/' => tokens.push(Token::ForwardSlash),
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

                    "Use" => Token::Use,
                    "As" => Token::As,
                    "Inherit" => Token::Inherit,

                    "Do" => Token::Do,
                    "Until" => Token::Until,
                    "If" => Token::If,
                    // "" => Token::,
                    _ => Token::Ident(s),
                });
            }

            x => panic!("{x}"),
        };
    }

    tokens
}

// this will always have atleast 1 subscope
#[derive(Clone, Debug)]
pub struct Scope {
    sub_scopes: Vec<SubScope>,
    str_to_class: HashMap<String, ClassId>,
    path_to_class: HashMap<Vec<String>, ClassId>,
    count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct SubScope {
    pub str_to_var: HashMap<String, VarId>,
    pub var_to_class: HashMap<VarId, ClassId>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            // 0, 1, and 2 are reserved
            count: 2,
            str_to_class: HashMap::from([
                (String::from("This"), ClassId::this()),
                (String::from("StringLiteral"), ClassId::string()),
                (String::from("NumberLiteral"), ClassId::int()),
            ]),
            path_to_class: HashMap::new(),
            sub_scopes: vec![SubScope {
                str_to_var: HashMap::new(),
                var_to_class: HashMap::new(),
            }],
        }
    }

    pub fn push_subscope(&mut self) {
        self.sub_scopes.push(SubScope {
            str_to_var: HashMap::new(),
            var_to_class: HashMap::new(),
        });
    }

    pub fn pop_subscope(&mut self) -> SubScope {
        assert!(self.sub_scopes.len() > 1);

        self.sub_scopes.pop().unwrap()
    }

    // fn last_subscope(mut self: Self) -> SubScope {
    //     assert_eq!(self.sub_scopes.len(), 1);
    //
    //     self.sub_scopes.pop().unwrap()
    // }

    pub fn var_insert(&mut self, name: String, class: ClassId) -> VarId {
        let subscope = self.sub_scopes.last_mut().unwrap();

        self.count += 1;

        let id = VarId(self.count);

        assert!(subscope.str_to_var.insert(name, id).is_none());
        assert!(subscope.var_to_class.insert(id, class).is_none());

        id
    }

    pub fn class_insert(&mut self, name: String, path: Vec<String>) -> ClassId {
        self.count += 1;

        let id = ClassId(self.count);

        // assert!(
        //     self.sub_scopes
        //         .last_mut()
        //         .unwrap()
        //         .str_to_class
        //         .insert(name, id)
        //         .is_none()
        // );

        assert!(self.path_to_class.insert(path, id).is_none());
        assert!(self.str_to_class.insert(name, id).is_none());

        id
    }

    pub fn get_var<'a>(&'a self, ident: &str) -> Option<VarId> {
        for subscope in self.sub_scopes.iter().rev() {
            if let x @ Some(_) = subscope.str_to_var.get(ident) {
                return x.cloned();
            }
        }

        None
    }

    pub fn get_class<'a>(&'a self, ident: &str) -> Option<ClassId> {
        self.str_to_class.get(ident).cloned()
    }

    pub fn type_var<'a>(&'a self, ident: VarId) -> Option<ClassId> {
        for subscope in self.sub_scopes.iter().rev() {
            if let x @ Some(_) = subscope.var_to_class.get(&ident) {
                return x.cloned();
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct TopLevelBlock {
    pub statements: Vec<Statement>,
    pub str_to_class: HashMap<String, ClassId>,
    pub path_to_class: HashMap<Vec<String>, ClassId>,
    pub subscope: SubScope,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub sub_scope: SubScope,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Method {
    pub args: Vec<(VarId)>,
    pub ret: Option<ClassId>,
    pub block: Block,
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum Statement {
    // TODO fix this
    // only top level
    Method(Method),
    Inherit(ClassId),
    Use(Use),

    // any depth
    Loop(Block, Value),
    If(Block, Value),
    MethodCall(MethodCall),
    VariableDeclaration(VarId, Value),
}

#[derive(Clone, Debug)]
pub struct Use {
    pub path: Vec<String>,
    pub name: ClassId,
}

#[derive(Clone, Debug)]
pub struct Value(pub ValueType);

#[derive(Clone, Debug)]
pub enum ValueType {
    Method(MethodCall),
    Constructor(ClassId, Vec<Value>),
    Variable(VarId),
    StringLiteral(String),
    NumberLiteral(f64),
    Undefined,
    Null,
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub args: Vec<Value>,
    pub method_name: String,
    pub target: Box<Value>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct VarId(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClassId(usize);

impl ClassId {
    pub fn this() -> Self {
        return ClassId(0);
    }

    pub fn int() -> Self {
        return ClassId(1);
    }

    pub fn string() -> Self {
        return ClassId(2);
    }
}

pub fn parse(tokens: &[Token]) -> TopLevelBlock {
    let mut scope = Scope::new();

    let (statements, 0) = parse_statements(&mut tokens.iter().peekable(), 0, &mut scope) else {
        unreachable!()
    };

    let Scope {
        mut sub_scopes,
        str_to_class,
        path_to_class,
        count: _,
    } = scope;

    let subscope = sub_scopes.pop().unwrap();

    assert!(sub_scopes.is_empty());

    TopLevelBlock {
        statements,
        str_to_class,
        path_to_class,
        subscope,
    }
}

fn parse_statements(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    depth: usize,
    scope: &mut Scope,
) -> (Vec<Statement>, usize) {
    let mut statements = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::String(_)) => {
                assert!(matches!(tokens.next().unwrap(), Token::String(_)));
            }

            Some(Token::Use) => {
                assert_eq!(*tokens.next().unwrap(), Token::Use);
                assert_eq!(depth, 0);

                let mut path = Vec::new();

                loop {
                    match tokens.next().unwrap() {
                        Token::Ident(x) => path.push(x.clone()),

                        Token::As => break,

                        _ => unreachable!(),
                    }
                }

                let Token::Ident(name) = tokens.next().unwrap().clone() else {
                    unreachable!()
                };

                statements.push(Statement::Use(Use {
                    name: scope.class_insert(name, path.clone()),
                    path,
                }));
            }

            Some(Token::Inherit) => {
                assert_eq!(*tokens.next().unwrap(), Token::Inherit);
                assert_eq!(depth, 0);

                let Token::Ident(name) = tokens.next().unwrap() else {
                    unreachable!()
                };

                statements.push(Statement::Inherit(scope.get_class(name).unwrap()));
            }

            Some(Token::Method) => {
                assert_eq!(*tokens.next().unwrap(), Token::Method);
                assert_eq!(depth, 0);

                let (args, block) = parse_method_definition(tokens, scope);

                statements.push(Statement::Method(Method {
                    args,
                    ret: Some(ClassId(0)),
                    block,
                    name: String::from("Constructor"),
                }))
            }

            Some(Token::Do) => {
                assert_eq!(*tokens.next().unwrap(), Token::Do);

                assert_eq!(*tokens.next().unwrap(), Token::NewLine);

                for _ in 0..=depth {
                    assert_eq!(*tokens.next().unwrap(), Token::Tab);
                }

                scope.push_subscope();

                let (loop_statements, ret_depth) = parse_statements(tokens, depth + 1, scope);

                assert_eq!(depth, ret_depth);

                assert_eq!(*tokens.next().unwrap(), Token::Until);

                let value = parse_value(tokens, scope);

                statements.push(Statement::Loop(
                    Block {
                        sub_scope: scope.pop_subscope(),
                        statements: loop_statements,
                    },
                    value,
                ));
            }

            Some(Token::If) => {
                assert_eq!(*tokens.next().unwrap(), Token::If);

                let value = parse_value(tokens, scope);

                assert_eq!(*tokens.next().unwrap(), Token::NewLine);

                scope.push_subscope();

                for _ in 0..=depth {
                    assert_eq!(*tokens.next().unwrap(), Token::Tab);
                }

                let (if_statements, ret_depth) = parse_statements(tokens, depth + 1, scope);

                statements.push(Statement::If(
                    Block {
                        sub_scope: scope.pop_subscope(),
                        statements: if_statements,
                    },
                    value,
                ));

                if dbg!(depth) != dbg!(ret_depth) {
                    return (statements, ret_depth);
                } else {
                    continue;
                }
            }

            Some(Token::LParens) => {
                let Value(ValueType::Method(call)) = parse_value(tokens, scope) else {
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
                    assert_eq!(depth, 0);

                    let (args, block) = parse_method_definition(tokens, scope);

                    statements.push(Statement::Method(Method {
                        args,
                        ret: None,
                        block,
                        name: name.clone(),
                    }));
                }
            }

            // either a method call
            Some(Token::Ident(_)) => {
                let token @ Token::Ident(name) = tokens.next().unwrap() else {
                    unreachable!()
                };

                match tokens.next().unwrap() {
                    Token::Dot => {
                        let Token::Ident(method_name) = tokens.next().unwrap() else {
                            unreachable!()
                        };

                        let args = parse_args(tokens, scope);

                        statements.push(Statement::MethodCall(MethodCall {
                            args,
                            method_name: method_name.clone(),
                            target: Box::new(Value(ValueType::Variable(
                                scope.get_var(name).unwrap(),
                            ))),
                        }));
                    }
                    Token::Ident(name2) => {
                        if Token::Method == **tokens.peek().unwrap() {
                            assert_eq!(depth, 0);

                            let (args, block) = parse_method_definition(tokens, scope);

                            statements.push(Statement::Method(Method {
                                args,
                                ret: Some(scope.get_class(name).unwrap()),
                                block,
                                name: name2.clone(),
                            }));
                        } else {
                            statements.push(Statement::VariableDeclaration(
                                scope.var_insert(name2.clone(), scope.get_class(name).unwrap()),
                                parse_value(tokens, scope),
                            ));
                        }
                    }

                    _ => unreachable!(),
                }
            }

            Some(Token::NewLine) => {
                // assert_eq!(*tokens.next().unwrap(), Token::NewLine);
                //
                // continue;
            }

            x @ Some(_) => unreachable!("{:?}, {:?}", x.cloned(), tokens),

            None => {
                // TODO is zero correct??
                return (
                    // Block {
                    statements,
                    //     sub_scope: scope.pop_subscope(),
                    // },
                    0,
                );
            }
        }

        if let Some(token) = tokens.next() {
            assert_eq!(*token, Token::NewLine, "{tokens:?}");
        } else {
            return (
                // Block {
                statements, //     sub_scope: scope.pop_subscope(),
                // },
                0,
            );
        }

        for i in 0..depth {
            if let Some(Token::Tab) = tokens.peek() {
                assert_eq!(*tokens.next().unwrap(), Token::Tab);
            } else if tokens.peek().is_some() && depth != 0 {
                return (statements, i);
            } else {
                return (statements, 0);
            }
        }
    }
}

fn parse_method_definition(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    scope: &mut Scope,
) -> (Vec<VarId>, Block) {
    assert_eq!(*tokens.next().unwrap(), Token::Method);

    let mut args = Vec::new();

    scope.push_subscope();

    while **tokens.peek().expect("method def should not end a file") != Token::NewLine {
        let Token::Ident(arg_class) = tokens.next().unwrap() else {
            unreachable!()
        };
        let Token::Ident(arg) = tokens.next().unwrap() else {
            unreachable!()
        };

        args.push(scope.var_insert(arg.clone(), scope.get_class(arg_class).unwrap()));
    }

    // let ret = if let Token::Ident(class) = ret {
    //     Some(scope.get_class(class).unwrap())
    // } else {
    //     assert_eq!(*ret, Token::Void);
    //     None
    // };

    assert_eq!(*tokens.next().unwrap(), Token::NewLine);

    // for _ in 0..=depth {
    assert_eq!(*tokens.next().unwrap(), Token::Tab);
    // }

    let (statements, ret_depth) = parse_statements(tokens, 1, scope);
    // let (statements, ret_depth) = parse_statements(tokens, depth + 1, scope);

    // assert_eq!(depth, ret_depth);
    assert_eq!(0, ret_depth);
    (
        args,
        Block {
            sub_scope: scope.pop_subscope(),
            statements,
        },
    )
}

fn parse_value(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    scope: &mut Scope,
) -> Value {
    let val = match tokens.next().unwrap() {
        Token::Ident(ident) => {
            if let Some(var) = scope.get_var(ident) {
                Value(ValueType::Variable(var))
            } else {
                Value(ValueType::Constructor(
                    scope.get_class(ident).unwrap(),
                    parse_args(tokens, scope),
                ))
            }
        }
        Token::LParens => {
            let val = parse_value(tokens, scope);
            assert_eq!(*tokens.next().unwrap(), Token::RParens);

            val
        }

        Token::String(x) => Value(ValueType::StringLiteral(x.clone())),
        Token::Number(x) => Value(ValueType::NumberLiteral(x.clone())),

        _ => unreachable!(),
    };

    // TODO change to while for chaining
    if Token::Dot == **tokens.peek().unwrap() {
        assert_eq!(Token::Dot, *tokens.next().unwrap());

        let Token::Ident(method_name) = tokens.next().unwrap() else {
            unreachable!()
        };

        let args = parse_args(tokens, scope);

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
fn parse_args(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    scope: &mut Scope,
) -> Vec<Value> {
    assert_eq!(*tokens.next().unwrap(), Token::LParens);

    let mut values = Vec::new();

    while Token::RParens != **tokens.peek().unwrap() {
        values.push(parse_value(tokens, scope));
    }

    assert_eq!(*tokens.next().unwrap(), Token::RParens);

    return values;
}
