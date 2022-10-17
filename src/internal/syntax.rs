/* DON'T execute cargo fmt or manually reformat */

use proc_macro2::Ident;
use syn::{
    token::{Semi, Let, Eq, Return, Paren, If, Brace, Else, Fn, Comma, Colon},
    punctuated::Punctuated,
    LitStr, LitBool, LitInt
};


pub(super) struct Program(
    pub Vec<Statement>
);
#[derive(Clone)]
pub enum Statement {
    Let{
        _let:       Let,
        ident:      Ident,
        _equal:     Eq,
        expr:       Expression,
        _semicolon: Semi,
    },
    Return{
        _return:    Return,
        expr:       Expression,
        _semicolon: Semi,
    },
    Expr{
        expr:       Expression,
        _semicolon: Semi,
    },
    IfElse{
        _if:       If,
        _paren:    Paren,
        condition: Expression,
        _brace:    Brace,
        process:   Vec<Statement>,

        after_if:  Option<ElseStatement>,
    },
}
    #[derive(Clone)]
    pub struct ElseStatement {
        pub _else:   Else,
        pub _brace:  Brace,
        pub process: Vec<Statement>,
    }

// pub struct Expression {
//     prefix:  Option<Prefix>,
//     _paren:  Option<Paren>,
//     content: Box<ExprInner>,
// }
    // pub enum ExprInner {
    //     Value(Value),
    //     Expr(Punctuated<Expression, Operator>),
    // }
    // pub struct ExprInner(
    //     pub Value,
    // );
#[derive(Clone)]
pub struct Expression(
    pub (ExprInner, Vec<(Operator, ExprInner)>),
);
    #[derive(Clone)]
    pub struct ExprInner {
        pub prefix:  Option<Prefix>,
        pub _paren:  Option<Paren>,
        pub content: (Value, Vec<(Operator, Value)>),
    }
        #[derive(Clone)]
        pub enum Value {
            Literal(Literal),
            Variable(Ident),
            FunctionCall{
                ident:      Ident,
                _paren:     Paren,
                args:       Punctuated<Expression, Comma>,
            },
            Hash{
                _brace:  Brace,
                content: Punctuated<KeyValue, Comma>,
            },
        }
            #[derive(Clone)]
            pub enum Literal {
                Int(LitInt),
                Bool(LitBool),
                Str(LitStr),
                Function{
                    _fn:     Fn,
                    _paren:  Paren,
                    args:    Punctuated<Ident, Comma>,
                    _brace:  Brace,
                    process: Vec<Statement>,
                },
            }
            #[derive(Clone)]
            pub struct KeyValue {
                pub key:    Expression,
                pub _colon: Colon,
                pub value:  Expression,
            }
        #[derive(Clone)]
        pub enum Operator {
            Plus,
            Minus,
            Mul,
            Div,
        }
    #[derive(Clone)]
    pub enum Prefix {
        Minus,
        Excram,
    }