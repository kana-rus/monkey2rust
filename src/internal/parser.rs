use proc_macro2::{Ident, Span, Punct};
use syn::{
    parse::{Parse, ParseBuffer},
    token, parenthesized, braced, Lit, Error,
};

use super::syntax::*;


impl Parse for Program {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Program({
            let mut vec = Vec::new();
            while !input.is_empty() {
                vec.push(input.parse()?)
            }
            vec
        }))
    }
}

impl Parse for Statement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(
            if input.peek(token::Let) {
                Statement::Let {
                    _let:       input.parse()?,
                    ident:      input.parse()?,
                    _equal:     input.parse()?,
                    expr:       input.parse()?,
                    _semicolon: input.parse()?,
                }
            } else if input.peek(token::Return) {
                Statement::Return {
                    _return:    input.parse()?,
                    expr:       input.parse()?,
                    _semicolon: input.parse()?,
                }
            } else if input.peek(token::If) {
                let condition_buf;
                let process_buf;
                Statement::IfElse {
                    _if: input.parse()?,
                    _paren: parenthesized!(condition_buf in input),
                    condition: condition_buf.parse()?,
                    _brace: braced!(process_buf in input),
                    process: {
                        let mut vec = Vec::new();
                        while !process_buf.is_empty() {
                            vec.push(process_buf.parse()?)
                        }
                        vec
                    },
                    after_if: {
                        if input.peek(token::Else) {
                            Some(input.parse()?)
                        } else {None}
                    },
                }
            } else {
                Statement::Expr {
                    expr:       input.parse()?,
                    _semicolon: input.parse()?
                }
            }
        )
    }
}
impl Parse for ElseStatement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let process_buf;
        Ok(ElseStatement {
            _else:   input.parse()?,
            _brace:  braced!(process_buf in input),
            process: {
                let mut vec = Vec::new();
                while !process_buf.is_empty() {
                    vec.push(process_buf.parse()?)
                }
                assert!(process_buf.is_empty(), "unexpected token at the end of 'else {{ }}'");
                vec
            },
        })
    }
}


fn parse_op_exprs(buf: & ParseBuffer) -> syn::Result<(ExprInner, Vec<(Operator, ExprInner)>)> {
    let fst = buf.parse().expect("in fst of op_exprs");// ?;
    let mut rest = Vec::new();
    while let Ok(op) = buf.parse() {
        rest.push((op, buf.parse().expect("in rest of op_exprs")))// ?))
    }
    Ok((fst, rest))
}
impl Parse for Expression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Expression(
            parse_op_exprs(&input)?,
        ))
    }
}
fn parse_op_values(buf: & ParseBuffer) -> syn::Result<(Value, Vec<(Operator, Value)>)> {
    let fst = buf.parse().expect("in fst of op_values");// ?;
    let mut rest = Vec::new();
    while let Ok(op) = buf.parse() {
        rest.push((op, buf.parse().expect("in rest of op_values")))// ?))
    }
    Ok((fst, rest))
}
impl Parse for ExprInner {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content_buf;
        let content;
        Ok(ExprInner {
            prefix: if input.peek(token::Bang) || input.peek(token::Sub) {
                Some(input.parse()?)} else {None},
            _paren: {
                if input.peek(token::Paren) {
                    let paren = parenthesized!(content_buf in input);
                    content = parse_op_values(&content_buf)?;
                    Some(paren)
                } else {
                    content = parse_op_values(input)?;
                    None
                }
            },
            content,
        })
    }
}
impl Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            let lit = input.parse::<Lit>()?;
            let value = Value::Literal(match lit {
                Lit::Bool(boolean) => Literal::Bool(boolean),
                Lit::Str(string) => Literal::Str(string),
                Lit::Int(int) => Literal::Int(int),
                _ => unreachable!("expeted (Bool | Str | Int) as literal")
            });
            Ok(value)
        } else if input.peek(token::Fn) {
            let args_buf;
            let process_buf;
            Ok(Value::Literal(Literal::Function {
                _fn:     input.parse()?,
                _paren:  parenthesized!(args_buf in input),
                args:    args_buf.parse_terminated(Ident::parse)?,
                _brace:  braced!(process_buf in input),
                process: {
                    let mut vec = Vec::new();
                    while !process_buf.is_empty() {
                        vec.push(process_buf.parse()?)
                    }
                    vec
                },
            }))
        } else if input.peek(syn::Ident) {
            if input.peek2(token::Paren) {
                let args_buf;
                Ok(Value::FunctionCall {
                    ident:  input.parse()?,
                    _paren: parenthesized!(args_buf in input),
                    args:   args_buf.parse_terminated(Expression::parse)?,
                })
            } else {
                Ok(Value::Variable(
                    input.parse()?,
                ))
            }
        } else if input.peek(token::Brace) {
            let content_buf;
            Ok(Value::Hash {
                _brace:  braced!(content_buf in input),
                content: content_buf.parse_terminated(KeyValue::parse)?,
            })
        } else {
            let this =
                if let Ok(punct) = input.parse::<Punct>() {
                    punct.to_string()
                } else {
                    "unknown".into()
                };
            Err(Error::new(Span::call_site(), format!("unexpected value: {this}")))
        }
    }
}
impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(KeyValue {
            key:    input.parse()?,
            _colon: input.parse()?,
            value:  input.parse()?,
        })
    }
}

impl Parse for Operator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Add)
        || input.peek(token::Sub)
        || input.peek(token::Star)
        || input.peek(token::Div) {
            match input.parse::<Punct>()?.as_char() {
                '+' => Ok(Operator::Plus),
                '-' => Ok(Operator::Minus),
                '*' => Ok(Operator::Mul),
                '/' => Ok(Operator::Div),
                 _ => unreachable!(),
            }
        } else {
            Err(Error::new(Span::call_site(), format!("unexpected operator")))
        }
    }
}
impl Parse for Prefix {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Bang)
        || input.peek(token::Sub) {
            match input.parse::<Punct>()?.as_char() {
                '!' => Ok(Prefix::Excram),
                '-' => Ok(Prefix::Minus),
                 _ => unreachable!()
            }
        } else {
            Err(Error::new(Span::call_site(), format!("unexpecetd prefix")))
        }
    }
}