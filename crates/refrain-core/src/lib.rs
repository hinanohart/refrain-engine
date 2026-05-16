//! refrain-core: Differential Refrain Engine core library.
//!
//! This crate defines the AST, public types, and parser entry points for the
//! Refrain DSL. The DSL is an S-expression language describing time-pattern
//! refrains and their compositional transformations.

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{Op, Pattern, Refrain};
pub use error::{RefrainError, Result};
pub use parser::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refrain_constructor_smoke() {
        let r = Refrain::new("melody-a");
        assert_eq!(r.name, "melody-a");
        assert!(r.territorialize.is_none());
        assert!(r.deterritorialize.is_none());
        assert!(r.reterritorialize.is_none());
    }

    #[test]
    fn ast_serde_roundtrip() {
        let mut r = Refrain::new("t");
        r.territorialize = Some(Pattern::Op(Op::Note {
            pitch: "C4".into(),
            dur: "q".into(),
        }));
        let s = serde_json::to_string(&r).unwrap();
        let r2: Refrain = serde_json::from_str(&s).unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn op_loop_holds_body() {
        let body = Box::new(Pattern::Op(Op::Note {
            pitch: "G4".into(),
            dur: "e".into(),
        }));
        let op = Op::Loop { count: 4, body };
        let s = serde_json::to_string(&op).unwrap();
        assert!(s.contains("Loop"));
        assert!(s.contains("\"count\":4"));
    }

    #[test]
    fn parser_not_yet_implemented() {
        let r = parser::parse("(refrain x)");
        assert!(matches!(r, Err(error::RefrainError::Parse(_))));
    }
}
