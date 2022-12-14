//! Contains all the core function that are common across
//! different modules.
use crate::kparser::KParserTracer;
use crate::kproc_macros::KTokenStream;
use crate::proc_macro::TokenTree;

use super::ast_nodes::StructTyToken;

/// parsing the declaration of the lifetimes and generics for a
/// declaration of a impl block or struct.
pub fn parse_decl_generics_and_lifetime(
    ast: &mut KTokenStream,
    _: &dyn KParserTracer,
) -> Option<StructTyToken> {
    if ast.match_tok("<") {
        let mut lifetimes = vec![];
        while !ast.match_tok(">") {
            let _ = ast.advance();
            if let Some(lifetime) = check_and_parse_lifetime(ast) {
                lifetimes.push(lifetime);
            }
            // FIXME: parse the generics types
            // FIXME: parse dyn token
            // FIXME: parse the declaration of types like `T: Sized`
            // FIXME: move the vector to map
        }
        let _ = ast.advance();
        let ty = StructTyToken {
            lifetimes,
            generics: vec![],
        };
        return Some(ty);
    }
    None
}

pub fn check_and_parse_ref<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek();
    match token.to_string().as_str() {
        "&" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

pub fn check_and_parse_lifetime<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "'" => {
            ast.next();
            Some(ast.advance().to_owned())
        }
        _ => None,
    }
}

pub fn check_and_parse_mut<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "mut" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

pub fn check_and_parse_dyn<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let token = ast.peek().to_string();
    match token.as_str() {
        "dyn" => Some(ast.advance().to_owned()),
        _ => None,
    }
}

/// parse visibility identifier like pub(crate) and return an option
/// value in case it is not defined.
///
/// FIXME: Return a AST type with a default value on private
/// to make the code cleaner.
pub fn parse_visibility_identifier<'c>(ast: &'c mut KTokenStream) -> Option<TokenTree> {
    let visibility = ast.peek();
    if let TokenTree::Ident(val) = visibility {
        if val.to_string().contains("pub") {
            return Some(ast.peek().to_owned());
        }
    }
    None
}
