//! refrain-egraph: equality-saturation normalization for Refrain ASTs.
//!
//! Uses the `egg` crate to define a `RefrainLang` term sort, applies a small
//! set of Refrain-specific rewrite rules, runs the e-graph to fixpoint
//! (bounded by a node limit and iteration cap), and extracts the
//! lowest-cost representative under the `AstSize` cost model.

use egg::{
    define_language, rewrite, AstSize, EGraph, Extractor, Id, RecExpr, Rewrite, Runner, Symbol,
};

use refrain_core::{Op, Pattern, Refrain, RefrainError, Result};

define_language! {
    /// Term sort for Refrain ASTs inside egg's e-graph.
    pub enum RefrainLang {
        "note"     = Note([Id; 2]),     // [pitch_sym, dur_sym]
        "loop"     = Loop([Id; 2]),     // [count_num, body]
        "dy/dx"    = Diff([Id; 2]),     // [x_sym, t_sym]
        "quotient" = Quotient(Box<[Id]>),
        "seq"      = Seq(Box<[Id]>),
        Num(u32),
        Sym(Symbol),
    }
}

/// The standard rewrite rule set for Refrain normalization.
fn rules() -> Vec<Rewrite<RefrainLang, ()>> {
    vec![
        rewrite!("loop-1-identity"; "(loop 1 ?x)" => "?x"),
        rewrite!("seq-singleton-identity"; "(seq ?x)" => "?x"),
    ]
}

pub struct Egraph {
    rules: Vec<Rewrite<RefrainLang, ()>>,
    node_limit: usize,
    iter_limit: usize,
}

impl Egraph {
    pub fn new() -> Self {
        Self {
            rules: rules(),
            node_limit: 10_000,
            iter_limit: 32,
        }
    }

    pub fn with_limits(node_limit: usize, iter_limit: usize) -> Self {
        Self {
            rules: rules(),
            node_limit,
            iter_limit,
        }
    }

    pub fn normalize(&self, r: &Refrain) -> Result<Refrain> {
        let mut out = Refrain::new(r.name.clone());
        out.territorialize = match &r.territorialize {
            Some(p) => Some(self.normalize_pattern(p)?),
            None => None,
        };
        out.deterritorialize = match &r.deterritorialize {
            Some(p) => Some(self.normalize_pattern(p)?),
            None => None,
        };
        out.reterritorialize = match &r.reterritorialize {
            Some(p) => Some(self.normalize_pattern(p)?),
            None => None,
        };
        Ok(out)
    }

    pub fn normalize_pattern(&self, p: &Pattern) -> Result<Pattern> {
        let mut expr = RecExpr::default();
        let _ = pattern_to_expr(p, &mut expr);
        let runner: Runner<RefrainLang, ()> = Runner::default()
            .with_node_limit(self.node_limit)
            .with_iter_limit(self.iter_limit)
            .with_expr(&expr);
        let runner = runner.run(&self.rules);
        let extractor = Extractor::new(&runner.egraph, AstSize);
        let root_id = runner.roots[0];
        let (_cost, best) = extractor.find_best(root_id);
        expr_to_pattern(&best)
    }
}

impl Default for Egraph {
    fn default() -> Self {
        Self::new()
    }
}

fn pattern_to_expr(p: &Pattern, b: &mut RecExpr<RefrainLang>) -> Id {
    match p {
        Pattern::Op(Op::Note { pitch, dur }) => {
            let ps = b.add(RefrainLang::Sym(Symbol::from(pitch.as_str())));
            let ds = b.add(RefrainLang::Sym(Symbol::from(dur.as_str())));
            b.add(RefrainLang::Note([ps, ds]))
        }
        Pattern::Op(Op::Loop { count, body }) => {
            let n = b.add(RefrainLang::Num(*count));
            let body_id = pattern_to_expr(body, b);
            b.add(RefrainLang::Loop([n, body_id]))
        }
        Pattern::Op(Op::Diff { x, t }) => {
            let xs = b.add(RefrainLang::Sym(Symbol::from(x.as_str())));
            let ts = b.add(RefrainLang::Sym(Symbol::from(t.as_str())));
            b.add(RefrainLang::Diff([xs, ts]))
        }
        Pattern::Op(Op::Quotient { rels }) => {
            let ids: Vec<Id> = rels
                .iter()
                .map(|s| b.add(RefrainLang::Sym(Symbol::from(s.as_str()))))
                .collect();
            b.add(RefrainLang::Quotient(ids.into_boxed_slice()))
        }
        Pattern::Op(Op::Sym(s)) => b.add(RefrainLang::Sym(Symbol::from(s.as_str()))),
        Pattern::Op(Op::Call { head, args }) => {
            let h = b.add(RefrainLang::Sym(Symbol::from(head.as_str())));
            let mut ids = vec![h];
            for a in args {
                ids.push(pattern_to_expr(a, b));
            }
            b.add(RefrainLang::Seq(ids.into_boxed_slice()))
        }
        Pattern::Seq(items) => {
            let ids: Vec<Id> = items.iter().map(|p| pattern_to_expr(p, b)).collect();
            b.add(RefrainLang::Seq(ids.into_boxed_slice()))
        }
    }
}

fn expr_to_pattern(expr: &RecExpr<RefrainLang>) -> Result<Pattern> {
    let nodes = expr.as_ref();
    let root = Id::from(nodes.len() - 1);
    node_to_pattern(nodes, root)
}

fn node_to_pattern(nodes: &[RefrainLang], id: Id) -> Result<Pattern> {
    let n = &nodes[usize::from(id)];
    match n {
        RefrainLang::Note([p, d]) => {
            let pitch = sym_at(nodes, *p)?.to_string();
            let dur = sym_at(nodes, *d)?.to_string();
            Ok(Pattern::Op(Op::Note { pitch, dur }))
        }
        RefrainLang::Loop([c, body]) => {
            let count = num_at(nodes, *c)?;
            let body_pat = node_to_pattern(nodes, *body)?;
            Ok(Pattern::Op(Op::Loop {
                count,
                body: Box::new(body_pat),
            }))
        }
        RefrainLang::Diff([x, t]) => {
            let xs = sym_at(nodes, *x)?.to_string();
            let ts = sym_at(nodes, *t)?.to_string();
            Ok(Pattern::Op(Op::Diff { x: xs, t: ts }))
        }
        RefrainLang::Quotient(ids) => {
            let mut rels = Vec::with_capacity(ids.len());
            for i in ids.iter() {
                rels.push(sym_at(nodes, *i)?.to_string());
            }
            Ok(Pattern::Op(Op::Quotient { rels }))
        }
        RefrainLang::Seq(ids) => {
            let mut items = Vec::with_capacity(ids.len());
            for i in ids.iter() {
                items.push(node_to_pattern(nodes, *i)?);
            }
            if items.len() == 1 {
                Ok(items.into_iter().next().unwrap())
            } else {
                Ok(Pattern::Seq(items))
            }
        }
        RefrainLang::Sym(s) => Ok(Pattern::Op(Op::Sym(s.as_str().to_string()))),
        RefrainLang::Num(n) => Ok(Pattern::Op(Op::Sym(n.to_string()))),
    }
}

fn sym_at(nodes: &[RefrainLang], id: Id) -> Result<&str> {
    match &nodes[usize::from(id)] {
        RefrainLang::Sym(s) => Ok(s.as_str()),
        other => Err(RefrainError::Rewrite(format!(
            "expected symbol, got {:?}",
            other
        ))),
    }
}

fn num_at(nodes: &[RefrainLang], id: Id) -> Result<u32> {
    match &nodes[usize::from(id)] {
        RefrainLang::Num(n) => Ok(*n),
        other => Err(RefrainError::Rewrite(format!(
            "expected number, got {:?}",
            other
        ))),
    }
}

/// Build a raw `EGraph` (without rules) for advanced use; the standard
/// pipeline goes through `Egraph::normalize`.
pub fn empty_egraph() -> EGraph<RefrainLang, ()> {
    EGraph::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use refrain_core::parse;

    #[test]
    fn loop_one_collapses_to_body() {
        let r = parse("(refrain a (territorialize (loop 1 (note C4 q))))").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        match n.territorialize.as_ref().unwrap() {
            Pattern::Op(Op::Note { pitch, dur }) => {
                assert_eq!(pitch, "C4");
                assert_eq!(dur, "q");
            }
            other => panic!("expected Note, got {:?}", other),
        }
    }

    #[test]
    fn loop_two_stays() {
        let r = parse("(refrain a (territorialize (loop 2 (note C4 q))))").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        match n.territorialize.as_ref().unwrap() {
            Pattern::Op(Op::Loop { count, .. }) => assert_eq!(*count, 2),
            other => panic!("expected Loop, got {:?}", other),
        }
    }

    #[test]
    fn note_stays_unchanged() {
        let r = parse("(refrain a (territorialize (note G4 e)))").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        assert_eq!(n, r);
    }

    #[test]
    fn diff_stays_unchanged() {
        let r = parse("(refrain a (deterritorialize (dy/dx intensity time)))").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        assert_eq!(n, r);
    }

    #[test]
    fn quotient_stays_unchanged() {
        let r = parse("(refrain a (reterritorialize (quotient ~a ~b)))").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        assert_eq!(n, r);
    }

    #[test]
    fn empty_refrain_normalizes_to_itself() {
        let r = parse("(refrain empty)").unwrap();
        let n = Egraph::default().normalize(&r).unwrap();
        assert_eq!(n, r);
    }

    #[test]
    fn normalize_is_idempotent() {
        let r = parse("(refrain a (territorialize (loop 1 (loop 1 (note C4 q)))))").unwrap();
        let n1 = Egraph::default().normalize(&r).unwrap();
        let n2 = Egraph::default().normalize(&n1).unwrap();
        assert_eq!(n1, n2);
    }
}
