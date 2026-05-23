use crate::parser::{
    Block, ClassId, Method, MethodCall, Scope, Statement, SubScope, TopLevelBlock, Use, Value,
    ValueType, VarId,
};

use std::{any::Any, collections::HashMap};

pub trait IO {}

// it has values that can be looked up
#[derive(Debug)]
struct Object(pub HashMap<String, ObjectEnum>);

#[derive(Clone, Debug)]
enum ObjectEnum {
    Object(ObjectId),

    Undefined,
    Null,

    // unless they cannot
    Any(AnyId),
    Number(f64),
    String(String),
}

#[derive(Clone, Copy, Debug)]
struct ObjectId(usize);

#[derive(Clone, Copy, Debug)]
struct AnyId(usize);

// pub fn interpret(statement: Vec<Statement>, scope: Scope, io: &mut dyn IO) {
//     todo!()
// }

// returns a method lookup table and a path to class look up table
fn load(
    block: TopLevelBlock,
    io: &mut dyn IO,
    id: ClassId,
) -> (
    // method name and sig -> Method
    HashMap<(String, Vec<ClassId>), Method>,
    // path -> ClassId
    HashMap<Vec<String>, ClassId>,
) {
    let TopLevelBlock {
        statements,
        str_to_class,
        path_to_class,
        subscope,
    } = block;

    (todo!(), path_to_class)
}

// not for any top level only
fn run(statement: Statement, scope: SubScope, values: HashMap<VarId, Object>, io: &mut dyn IO) {
    match statement {
        Statement::Loop(block, value) => todo!(),
        Statement::If(block, value) => todo!(),
        Statement::MethodCall(MethodCall {
            args,
            method_name,
            target,
        }) => {}
        Statement::VariableDeclaration(var_id, value) => todo!(),
        Statement::Method(_) | Statement::Inherit(_) | Statement::Use(_) => unreachable!(),
    }
}

fn run_val(
    value: Value,
    values: HashMap<VarId, ObjectEnum>,
    objects: HashMap<ObjectId, Object>,
    methods: HashMap<(ClassId, String, Vec<ClassId>), Method>,
    io: &mut dyn IO,
) -> ObjectEnum {
    match value.0 {
        ValueType::Method(MethodCall {
            args,
            method_name,
            target,
        }) => {}
        ValueType::Constructor(class_id, values) => todo!(),
        ValueType::Variable(var_id) => values.get(&var_id).unwrap().clone(),
        ValueType::StringLiteral(x) => ObjectEnum::String(x),
        ValueType::NumberLiteral(x) => ObjectEnum::Number(x),
        ValueType::Undefined => ObjectEnum::Undefined,
        ValueType::Null => ObjectEnum::Null,
    }
}
