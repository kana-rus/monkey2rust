use proc_macro2::TokenStream;
use quote::quote;
use super::syntax::*;


impl Into<TokenStream> for Program {
    fn into(self) -> TokenStream {
        let mut main_process = TokenStream::new();
        for stmt in self.0 {
            main_process.extend::<TokenStream>(stmt.into())   
        }
        quote!(
            fn main() {
                #main_process
            }
        )
    }
}
impl Into<TokenStream> for Statement {
    fn into(self) -> TokenStream {
        match self {
            Statement::Expr {expr, _semicolon} => expr.into(),
            Statement::Let {
                _let, ident, _equal, expr, _semicolon
            } => {
                let rust_expr: TokenStream = expr.into();
                quote!(
                    let #ident = #rust_expr;
                )
            },
            Statement::Return {
                _return, expr, _semicolon
            } => {
                let rust_expr: TokenStream = expr.into();
                quote!(
                    return #rust_expr;
                )
            },
            Statement::IfElse {
                _if, _paren, condition, _brace, process, after_if
            } => {
                let rust_condition: TokenStream = condition.into();
                let rust_process = {
                    let mut rust_process = TokenStream::new();
                    for p in process {
                        rust_process.extend::<TokenStream>(p.into())
                    }
                    rust_process
                };

                if let Some(else_stmt) = after_if {
                    let rust_else_process = {
                        let mut rust_else_process = TokenStream::new();
                        for p in else_stmt.process {
                            rust_else_process.extend::<TokenStream>(p.into())
                        }
                        rust_else_process
                    };
                    quote!(
                        if #rust_condition {
                            #rust_process
                        } else {
                            #rust_else_process
                        }
                    )
                } else {
                    quote!(
                        if #rust_condition {
                            #rust_process
                        }
                    )
                }
            },
        }
    }
}

impl Into<TokenStream> for Expression {
    fn into(self) -> TokenStream {
        let mut rust_expr = TokenStream::new();
        let (fst, rest) = self.0;

        rust_expr.extend::<TokenStream>(fst.into());
        for (op, expr) in rest {
            rust_expr.extend::<TokenStream>(op.into());
            rust_expr.extend::<TokenStream>(expr.into());
        }
        rust_expr
    }
}
impl Into<TokenStream> for ExprInner {
    fn into(self) -> TokenStream {
        let mut rust_expr_inner = TokenStream::new();

        match self.prefix {
            None => (),
            Some(prefix) => rust_expr_inner.extend::<TokenStream>(prefix.into()),
        }

        let mut core = TokenStream::new();
        let (fst, rest) = self.content;
        core.extend::<TokenStream>(fst.into());
        for (op, v) in rest {
            core.extend::<TokenStream>(op.into());
            core.extend::<TokenStream>(v.into());
        }

        rust_expr_inner.extend(
            if self._paren.is_some() {quote!(
                (#core)
            )} else {core}
        );

        rust_expr_inner
    }
}
impl Into<TokenStream> for Value {
    fn into(self) -> TokenStream {
        match self {
            Value::Variable(name) => quote!(#name),
            Value::Literal(literal) => literal.into(),
            Value::FunctionCall {
                ident,
                _paren,
                args
            } => {
                let mut rust_args = TokenStream::new();
                for arg in args {
                    rust_args.extend::<TokenStream>(arg.into())
                }
                quote!(
                    #ident(#rust_args)
                )
            },
            Value::Hash {
                _brace,
                content
            } => panic!("Hash map is not supported in current version!")
        }
    }
}
impl Into<TokenStream> for Literal {
    fn into(self) -> TokenStream {
        match self {
            Literal::Int(lit_int) => quote!(#lit_int),
            Literal::Bool(lit_bool) => quote!(#lit_bool),
            Literal::Str(lit_str) => quote!(#lit_str),
            Literal::Function {
                _fn,
                _paren,
                args,
                _brace,
                process
            } => {
                let mut rust_process = TokenStream::new();
                for p in process {
                    rust_process.extend::<TokenStream>(p.into());
                }
                quote!(
                    |#args| {
                        #rust_process
                    }
                )
            }
        }
    }
}
/*
    impl Into<TokenStream> for KeyValue {
        fn into(self) -> TokenStream {

        }
    }
*/
impl Into<TokenStream> for Operator {
    fn into(self) -> TokenStream {
        match self {
            Operator::Plus => quote!(+),
            Operator::Minus => quote!(-),
            Operator::Mul => quote!(*),
            Operator::Div => quote!(/),
        }
    }
}
impl Into<TokenStream> for Prefix {
    fn into(self) -> TokenStream {
        match self {
            Prefix::Excram => quote!(!),
            Prefix::Minus => quote!(-),
        }
    }
    
}