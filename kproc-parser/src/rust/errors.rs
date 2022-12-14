//! Error generated by the parser
//! that it is caused by a syntax
//! error.
//!
//! Useful to print a compiler message.
use crate::proc_macro::TokenTree;

#[derive(Debug)]
pub struct SyntaxError {
    tok: TokenTree,
    msg: String,
}
