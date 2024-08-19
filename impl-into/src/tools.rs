use crate::prelude::*;
use proc_macro2::{ TokenStream, TokenTree };
use venial::Attribute;

// Check attribute name
pub(crate) fn check_attr_name(attr: &Attribute, name: &str) -> bool {
    if let Some(attr_name) = attr.get_single_path_segment() {
        if attr_name == name {
            return true;
        }
    };
    false
}

// Parsing the attribute single argument
pub(crate) fn parse_attr_argument(token: &TokenTree) -> Result<TokenStream> {
    let expr: TokenStream = if let TokenTree::Literal(lit) = &token {
        let text = lit.to_string();
        text[1..text.len()-1].replace("\\\"", "\"").parse().unwrap()
    } else {
        return Err(Error::IncorrectAttribute)
    };

    Ok(expr)
}

// Parsing attribute arguments to ("type.." = "expr..")
pub(crate) fn parse_attr_arguments(tokens: &Vec<TokenTree>) -> Result<(TokenStream, TokenStream)> {
    if tokens.len() != 3 { return Err(Error::IncorrectAttribute) }
    
    let ty: TokenStream = if let TokenTree::Literal(lit) = &tokens[0] {
        let text = lit.to_string();
        text[1..text.len()-1].parse().unwrap()
    } else {
        return Err(Error::IncorrectAttribute)
    };

    let expr: TokenStream = if let TokenTree::Literal(lit) = &tokens[2] {
        let text = lit.to_string();
        text[1..text.len()-1].replace("\\\"", "\"").parse().unwrap()
    } else {
        return Err(Error::IncorrectAttribute)
    };

    Ok((ty, expr))
}
