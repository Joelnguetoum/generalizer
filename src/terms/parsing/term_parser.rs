use std::str::Chars;
use crate::terms::function::{Function, Signature};

use crate::terms::term::Term;

/// Main entry point
pub fn parse_term(sig: &Signature, term_string: &String) -> Result<Term, String> {
    let mut chars = term_string.chars().peekable();
    let term = parse_from_chars(sig, &mut chars)?;
    if chars.peek().is_some() {
        return Err("Unexpected trailing characters".to_string());
    }
    Ok(term)
}

fn parse_from_chars(
    sig: &Signature,
    chars: &mut std::iter::Peekable<Chars<'_>>,
) -> Result<Term, String> {
    let name = parse_identifier(chars)?;

    if let Some('(') = chars.peek() {
        // function with arguments
        chars.next(); // consume '('
        let mut args = Vec::new();

        if let Some(')') = chars.peek() {
            chars.next(); // consume ')'
        } else {
            loop {
                let arg = parse_from_chars(sig, chars)?;
                args.push(arg);
                match chars.next() {
                    Some(',') => continue,
                    Some(')') => break,
                    Some(c) => return Err(format!("Unexpected character '{}'", c)),
                    None => return Err("Unexpected end of input".to_string()),
                }
            }
        }

        // check arity
        if let Some(fsig) = sig.iter().find(|s| s.name == name) {
            if fsig.arity != args.len() {
                return Err(format!(
                    "Arity mismatch for function '{}': expected {}, got {}",
                    fsig.name, fsig.arity, args.len()
                ));
            }
            Ok(Term::Function(Function {
                signature: fsig.clone(),
                args,
            }))
        } else {
            Err(format!("Unknown function '{}'", name))
        }
    } else {
        // bare identifier → must be a 0-arity function
        if let Some(fsig) = sig.iter().find(|s| s.name == name) {
            if fsig.arity != 0 {
                return Err(format!(
                    "Function '{}' used without arguments but has arity {}",
                    fsig.name, fsig.arity
                ));
            }
            Ok(Term::Function(Function {
                signature: fsig.clone(),
                args: Vec::new(),
            }))
        } else {
            Err(format!("Unknown function '{}'", name))
        }
    }
}

fn parse_identifier(chars: &mut std::iter::Peekable<Chars<'_>>) -> Result<String, String> {
    let mut ident = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            ident.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if ident.is_empty() {
        Err("Expected identifier".to_string())
    } else {
        Ok(ident)
    }
}