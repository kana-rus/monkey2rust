/* DON'T execute cargo fmt or manually reformat */

use proc_macro2::Ident;
use syn::{token::{Semi, Let, Eq, Return, Paren, If, Brace, Else, Fn, Comma, Colon}, punctuated::Punctuated, LitStr, LitBool, LitInt};


pub struct Program(Vec<Statement>);

enum Statement {
    Let{
        _let:       Let,
        ident:      Ident,
        _equal:     Eq,
        expr:       MonkeyExpr,
        _semicolon: Semi,
    },
    Return{
        _return:    Return,
        expr:       MonkeyExpr,
        _semicolon: Semi,
    },
    Expr{
        expr:       MonkeyExpr,
        _semicolon: Semi,
    },
    IfElse{
        _if:       If,
        _paren:    Paren,
        condition: MonkeyExpr,
        _brace:    Brace,
        process:   Vec<Statement>,

        after_if:  Option<ElseStatement>,
    },
}
    struct ElseStatement {
        _else: Else,
        after_else: AfterElse,
    }
    enum AfterElse {
        IfStatement,
        ElseExpr{
            _brace:  Brace,
            process: Vec<Statement>,
        },
    }

struct MonkeyExpr {
    prefix:  Option<Prefix>,
    _paren: Option<Paren>,
    content: Box<ExprInner>,
}
    enum ExprInner {
        Value(MonkeyValue),
        Expr(Punctuated<MonkeyExpr, Operator>),
    }
        enum MonkeyValue {
            Literal(Literal),
            Variable(Ident),
            FunctionCall{
                ident:      Ident,
                _paren:     Paren,
                args:       Punctuated<MonkeyExpr, Comma>,
            },
        }
            enum Literal {
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
                Hash{
                    _brace:  Brace,
                    content: Punctuated<KeyValue, Comma>,
                },
            }
                struct KeyValue {
                    key:    MonkeyExpr,
                    _colon: Colon,
                    value:  MonkeyExpr,
                }
        enum Operator {
            Plus,
            Minus,
            Mul,
            Div,
        }
    enum Prefix {
        Minus,
        Excram,
    }