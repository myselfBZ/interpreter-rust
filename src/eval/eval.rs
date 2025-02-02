use crate::ast::ast::{self, Expression};
use crate::objects::objects;

pub fn eval_program(program:Vec<ast::Statement>) -> objects::Object {
    let mut result: objects::Object = objects::Object::Null;
    for s in program{
        result = eval_stmnt(s); 
    }
    result
}

fn eval_stmnt(stmnt:ast::Statement) -> objects::Object {
    match stmnt {
        ast::Statement::ExprsStatement { token: _, exprs } => eval_exprs(exprs),
        _ => objects::Object::Null
    }
}

fn eval_exprs(s:ast::Expression) -> objects::Object  {
    match s {
        ast::Expression::Int(s) => objects::Object::Int(s),
        ast::Expression::Boolean{token: _, value} => objects::Object::Bool(value),
        ast::Expression::InfixExprsn { left, right, oprt } => eval_infix(*left, *right, oprt),
        _ =>  objects::Object::Null
    }
}

fn eval_infix(left:ast::Expression, right:Expression, oprtr:String) -> objects::Object {
        let right = eval_exprs(right);
        let left = eval_exprs(left);
        match (right, left) {
            (objects::Object::Int(a), objects::Object::Int(b)) => {
                match oprtr.as_str(){
                    "+" => objects::Object::Int(a + b),
                    "-" => objects::Object::Int(a - b),
                    "*" => objects::Object::Int(a * b),
                    "/" => objects::Object::Int(a / b),
                    _ => objects::Object::Null
                }
            },
            _ => {
                println!("oops mismatched types");
                objects::Object::Null
        }
    }
}
