//! Hand-rolled S-expression parser for the Refrain DSL.
//!
//! Grammar (informal):
//!
//! ```text
//! refrain := "(" "refrain" IDENT stage* ")"
//! stage   := "(" stage-kind pattern+ ")"
//! stage-kind := "territorialize" | "deterritorialize" | "reterritorialize"
//! pattern := ATOM | "(" op-head pattern* ")"
//! op-head := "note" | "loop" | "dy/dx" | "quotient" | IDENT
//! ```
//!
//! Sequences of patterns inside a stage become `Pattern::Seq`. Unknown
//! function-like forms become `Op::Call { head, args }` so the parser does
//! not have to be updated for every future operator.

use std::iter::Peekable;

use crate::ast::{Op, Pattern, Refrain};
use crate::error::{RefrainError, Result};

#[derive(Debug, Clone, PartialEq)]
enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

pub fn parse(src: &str) -> Result<Refrain> {
    let tokens = tokenize(src);
    let mut iter = tokens.into_iter().peekable();
    let s = parse_sexp(&mut iter)?;
    if iter.next().is_some() {
        return Err(RefrainError::Parse(
            "trailing tokens after top-level form".into(),
        ));
    }
    sexp_to_refrain(&s)
}

fn tokenize(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut in_comment = false;
    for ch in src.chars() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
            }
            continue;
        }
        if ch == ';' {
            flush(&mut cur, &mut tokens);
            in_comment = true;
            continue;
        }
        if ch.is_whitespace() {
            flush(&mut cur, &mut tokens);
            continue;
        }
        if ch == '(' || ch == ')' {
            flush(&mut cur, &mut tokens);
            tokens.push(ch.to_string());
            continue;
        }
        cur.push(ch);
    }
    flush(&mut cur, &mut tokens);
    tokens
}

fn flush(cur: &mut String, out: &mut Vec<String>) {
    if !cur.is_empty() {
        out.push(std::mem::take(cur));
    }
}

fn parse_sexp<I: Iterator<Item = String>>(iter: &mut Peekable<I>) -> Result<Sexp> {
    let tok = iter
        .next()
        .ok_or_else(|| RefrainError::Parse("unexpected EOF".into()))?;
    if tok == "(" {
        let mut items = Vec::new();
        loop {
            let peek = iter
                .peek()
                .ok_or_else(|| RefrainError::Parse("unclosed list".into()))?;
            if peek == ")" {
                iter.next();
                return Ok(Sexp::List(items));
            }
            items.push(parse_sexp(iter)?);
        }
    } else if tok == ")" {
        Err(RefrainError::Parse("unexpected `)` at top".into()))
    } else {
        Ok(Sexp::Atom(tok))
    }
}

fn sexp_to_refrain(s: &Sexp) -> Result<Refrain> {
    let items = expect_list(s, "refrain form")?;
    if items.len() < 2 {
        return Err(RefrainError::Parse(
            "refrain form requires `refrain` head and a name".into(),
        ));
    }
    let head = expect_atom(&items[0], "refrain head")?;
    if head != "refrain" {
        return Err(RefrainError::Parse(format!(
            "expected `refrain`, got `{}`",
            head
        )));
    }
    let name = expect_atom(&items[1], "refrain name")?.to_string();
    let mut r = Refrain::new(name);
    for stage in &items[2..] {
        let stage_items = expect_list(stage, "stage form")?;
        if stage_items.is_empty() {
            return Err(RefrainError::Parse("empty stage form".into()));
        }
        let kind = expect_atom(&stage_items[0], "stage kind")?;
        let body = sexp_to_pattern_body(&stage_items[1..])?;
        match kind {
            "territorialize" => r.territorialize = Some(body),
            "deterritorialize" => r.deterritorialize = Some(body),
            "reterritorialize" => r.reterritorialize = Some(body),
            other => {
                return Err(RefrainError::Parse(format!("unknown stage `{}`", other)));
            }
        }
    }
    Ok(r)
}

fn sexp_to_pattern_body(items: &[Sexp]) -> Result<Pattern> {
    match items.len() {
        0 => Err(RefrainError::Parse("empty stage body".into())),
        1 => sexp_to_pattern(&items[0]),
        _ => {
            let mut pats = Vec::with_capacity(items.len());
            for i in items {
                pats.push(sexp_to_pattern(i)?);
            }
            Ok(Pattern::Seq(pats))
        }
    }
}

fn sexp_to_pattern(s: &Sexp) -> Result<Pattern> {
    match s {
        Sexp::Atom(a) => Ok(Pattern::Op(Op::Sym(a.clone()))),
        Sexp::List(xs) => {
            if xs.is_empty() {
                return Err(RefrainError::Parse("empty pattern list".into()));
            }
            let head = expect_atom(&xs[0], "op head")?;
            match head {
                "note" => {
                    if xs.len() != 3 {
                        return Err(RefrainError::Parse(
                            "note expects (note PITCH DUR)".into(),
                        ));
                    }
                    let pitch = expect_atom(&xs[1], "note pitch")?.to_string();
                    let dur = expect_atom(&xs[2], "note dur")?.to_string();
                    Ok(Pattern::Op(Op::Note { pitch, dur }))
                }
                "loop" => {
                    if xs.len() != 3 {
                        return Err(RefrainError::Parse(
                            "loop expects (loop COUNT BODY)".into(),
                        ));
                    }
                    let count_atom = expect_atom(&xs[1], "loop count")?;
                    let count: u32 = count_atom.parse().map_err(|_| {
                        RefrainError::Parse(format!("loop count `{}` not u32", count_atom))
                    })?;
                    let body = Box::new(sexp_to_pattern(&xs[2])?);
                    Ok(Pattern::Op(Op::Loop { count, body }))
                }
                "dy/dx" => {
                    if xs.len() != 3 {
                        return Err(RefrainError::Parse(
                            "dy/dx expects (dy/dx X T)".into(),
                        ));
                    }
                    let x = expect_atom(&xs[1], "dy/dx x")?.to_string();
                    let t = expect_atom(&xs[2], "dy/dx t")?.to_string();
                    Ok(Pattern::Op(Op::Diff { x, t }))
                }
                "quotient" => {
                    let mut rels = Vec::new();
                    for s in &xs[1..] {
                        rels.push(expect_atom(s, "quotient rel")?.to_string());
                    }
                    Ok(Pattern::Op(Op::Quotient { rels }))
                }
                _ => {
                    let head_owned = head.to_string();
                    let mut args = Vec::with_capacity(xs.len() - 1);
                    for s in &xs[1..] {
                        args.push(sexp_to_pattern(s)?);
                    }
                    Ok(Pattern::Op(Op::Call {
                        head: head_owned,
                        args,
                    }))
                }
            }
        }
    }
}

fn expect_list<'a>(s: &'a Sexp, what: &str) -> Result<&'a [Sexp]> {
    match s {
        Sexp::List(xs) => Ok(xs),
        Sexp::Atom(_) => Err(RefrainError::Parse(format!("expected list ({})", what))),
    }
}

fn expect_atom<'a>(s: &'a Sexp, what: &str) -> Result<&'a str> {
    match s {
        Sexp::Atom(a) => Ok(a.as_str()),
        Sexp::List(_) => Err(RefrainError::Parse(format!("expected atom ({})", what))),
    }
}
