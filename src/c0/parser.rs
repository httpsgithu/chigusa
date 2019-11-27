use super::ast::Literal;
use super::ast::*;
use super::err::*;
use super::lexer::*;
use crate::prelude::*;
use bimap::BiMap;
use std::iter::Iterator;
use std::iter::Peekable;

use either::Either;
use LoopCtrl::*;

pub trait IntoParser {
    fn into_parser(self) -> Parser;
}

impl IntoParser for Lexer {
    fn into_parser(self) -> Parser {
        Parser::new(self)
    }
}

// pub trait TokenIterator: Iterator<Item = Token> + itertools::PeekingNext {
//     fn expect(&mut self, token: TokenVariant) -> ParseResult<Token> {
//         // * separated variables because lifetime concerns.
//         match self.next() {
//             Some(t) => {
//                 if variant_eq(&t.var, &token) {
//                     Ok(t)
//                 } else {
//                     Err(parse_err(ParseErrVariant::ExpectToken(token), t.span))
//                 }
//             }
//             None => Err(parse_err(ParseErrVariant::ExpectToken(token), Span::zero())),
//         }
//     }

//     fn expect_peek(&mut self, token: TokenVariant) -> ParseResult<Token> {
//         // * separated variables because lifetime concerns.
//         match self.peeking_next(|_| false) {
//             Some(t) => {
//                 if variant_eq(&t.var, &token) {
//                     Ok(t)
//                 } else {
//                     Err(parse_err(ParseErrVariant::ExpectToken(token), t.span))
//                 }
//             }
//             None => Err(parse_err(ParseErrVariant::ExpectToken(token), Span::zero())),
//         }
//     }

//     fn expect_map_or<T>(
//         &mut self,
//         token: TokenVariant,
//         map: impl FnOnce(Token) -> T,
//         f: impl FnOnce(Token) -> Result<T, ParseError>,
//     ) -> ParseResult<T> {
//         let next = self.next();
//         match next {
//             Some(v) => {
//                 if variant_eq(&v.var, &token) {
//                     Ok(map(v))
//                 } else {
//                     f(v)
//                 }
//             }
//             None => Err(parse_err(ParseErrVariant::ExpectToken(token), Span::zero())),
//         }
//     }

//     fn try_consume(&mut self, token: TokenVariant) -> bool {
//         match self.peeking_next(|v| variant_eq(&v.var, &token)) {
//             Some(_) => true,
//             None => false,
//         }
//     }

//     fn try_consume_log_span(&mut self, token: TokenVariant) -> Option<Span> {
//         match self.peeking_next(|v| variant_eq(&v.var, &token)) {
//             Some(v) => Some(v.span),
//             None => None,
//         }
//     }
// }

// type LexerWrapped = Peekable<Lexer>;

// impl TokenIterator for LexerWrapped {}

pub struct TypeVar {
    types: Vec<TypeDef>,
    type_names: BiMap<usize, String>,
    // vars: Vec<VarDef>,
    // var_names: BiMap<usize, String>,
}

impl TypeVar {
    pub fn new() -> TypeVar {
        TypeVar {
            types: Vec::new(),
            type_names: BiMap::new(),
            // vars: Vec::new(),
            // var_names: BiMap::new(),
        }
    }

    pub fn insert_type(&mut self, type_name: &str, type_def: TypeDef) -> usize {
        unimplemented!()
    }
}

pub struct Parser {
    lexer: Lexer,
    type_var: TypeVar,
    cur: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let first_tok = lexer.next();
        let mut parser = Parser {
            lexer: lexer,
            type_var: TypeVar::new(),
            cur: Token::dummy(),
        };
        parser.cur = parser.lexer.next().unwrap();
        parser
    }

    fn bump(&mut self) {
        self.cur = self.lexer.next().unwrap_or_else(|| Token::eof());
    }

    fn check(&self, accept: &TokenType) -> bool {
        variant_eq(&self.cur.var, accept)
    }

    fn expect(&mut self, accept: &TokenType) -> bool {
        if self.check(accept) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect_report(&mut self, accept: &TokenType) -> ParseResult<()> {
        if self.expect(accept) {
            Ok(())
        } else {
            Err(parse_err(
                // We used clone here, because once we meet an error we no longer
                // need to worry about performance. Things're gonna fail anyway.
                ParseErrVariant::ExpectToken(accept.clone()),
                self.cur.span,
            ))
        }
    }

    fn check_one_of(&mut self, accept: &[TokenType]) -> bool {
        accept.iter().any(|x| variant_eq(&self.cur.var, &x))
    }

    fn expect_one_of(&mut self, accept: &[TokenType]) -> bool {
        if self.check_one_of(accept) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect_one_of_report(&mut self, accept: &[TokenType]) -> ParseResult<()> {
        if self.expect_one_of(accept) {
            Ok(())
        } else {
            Err(parse_err(
                // We used clone here, because once we meet an error we no longer
                // need to worry about performance. Things're gonna fail anyway.
                ParseErrVariant::ExpectTokenOneOf(accept.iter().map(|x| x.clone()).collect()),
                self.cur.span,
            ))
        }
    }

    pub fn parse(&mut self) -> ParseResult<Program> {
        unimplemented!();
        // Ok(Program {
        //     scope: (),
        //     vars: (),
        //     types: (),
        // })
    }

    fn p_stmt_or_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Either<Stmt, Expr>> {
        unimplemented!()
    }

    fn p_block_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        unimplemented!()
    }

    fn p_block_expr_no_scope(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        unimplemented!()
    }

    fn p_fn(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        unimplemented!()
    }

    fn p_while_stmt(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        unimplemented!()
    }

    fn p_if_stmt(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        let mut span = self.cur.span;

        self.expect_report(&TokenType::If)?;
        todo!("We are refactoring this thing")

        // let cond = Ptr::new(self.p_expr(scope.clone())?);
        // let if_block = Ptr::new(if self.lexer.expect_peek(TokenType::LCurlyBrace).is_ok() {
        //     self.p_block_expr(scope.clone())
        // } else {
        //     self.p_expr(scope.clone())
        // }?);

        // span = span + if_block.borrow().span();
        // let else_span = self.lexer.try_consume_log_span(TokenType::Else);
        // let else_block = if else_span.is_some() {
        //     Some(Ptr::new(
        //         if self.lexer.expect_peek(TokenType::LCurlyBrace).is_ok() {
        //             self.p_block_expr(scope.clone())
        //         } else {
        //             self.p_expr(scope.clone())
        //         }?,
        //     ))
        // } else {
        //     None
        // };

        // else_block.as_ref().map(|e| span = span + e.borrow().span());

        // Ok(Expr {
        //     var: ExprVariant::IfConditional(IfConditional {
        //         cond,
        //         if_block,
        //         else_block,
        //     }),
        //     span,
        // })
    }
    fn p_decl_stmt(&mut self, scope: Ptr<Scope>) -> ParseResult<Stmt> {
        todo!()
    }

    fn p_expr(&mut self, scope: Ptr<Scope>) -> ParseResult<Expr> {
        let stack = Vec::<Expr>::new();
        todo!();
    }

    /// Parses a binary operator with at least the precedence specified.
    ///
    /// Design stolen from https://github.com/rust-lang/rust/blob/b5f265eeed23ac87ec6b4a7e6bc7cb4ea3e67c31/src/librustc_parse/parser/expr.rs#L141
    fn p_binary_op(&mut self, expect_prec: i32, scope: Ptr<Scope>) -> ParseResult<Expr> {
        todo!("Check out rustc's official implementation!")
    }

    fn p_unary_op(&mut self, expect_prec: i32, scope: Ptr<Scope>) -> ParseResult<Expr> {
        todo!()
    }

    fn p_literal(&mut self) -> ParseResult<Expr> {
        let t = self.lexer.next().unwrap();
        match t.var {
            TokenType::Literal(i) => Ok(Expr {
                var: ExprVariant::Literal(i.into()),
                span: t.span,
            }),
            v @ _ => Err(parse_err(
                ParseErrVariant::InternalErr(format!(
                    "Bad branching into literal parsing while getting a token type of {}",
                    v
                )),
                t.span,
            )),
        }
    }
}
