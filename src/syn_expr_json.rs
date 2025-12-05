//! Comprehensive JSON serialization for syn::Expr.
//!
//! This module provides a structured JSON serialization for all `syn::Expr` variants
//! without relying on Debug formatting. Complex nested nodes (Types, Pats, Attributes,
//! Items, generic args, token fragments) are converted to stable source-like strings
//! using ToTokens.

use quote::ToTokens;
use serde_json::{json, Value};
use syn::{
    Arm, BinOp, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock,
    ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue, ExprField, ExprForLoop,
    ExprGroup, ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch,
    ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprRawAddr, ExprReference, ExprRepeat,
    ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprUnary, ExprUnsafe, ExprWhile,
    ExprYield, FieldValue, Index, Label, Lit, Member, Pat, PointerMutability, RangeLimits, Type, UnOp,
};

/// Convert a syn::Expr to a serde_json::Value with structured JSON.
///
/// This function exhaustively pattern-matches all syn::Expr variants and serializes
/// them into a structured JSON format. It does not use Debug formatting.
pub fn expr_to_json(expr: &Expr) -> Value {
    match expr {
        Expr::Array(e) => array_to_json(e),
        Expr::Assign(e) => assign_to_json(e),
        Expr::Async(e) => async_to_json(e),
        Expr::Await(e) => await_to_json(e),
        Expr::Binary(e) => binary_to_json(e),
        Expr::Block(e) => block_to_json(e),
        Expr::Break(e) => break_to_json(e),
        Expr::Call(e) => call_to_json(e),
        Expr::Cast(e) => cast_to_json(e),
        Expr::Closure(e) => closure_to_json(e),
        Expr::Const(e) => const_to_json(e),
        Expr::Continue(e) => continue_to_json(e),
        Expr::Field(e) => field_to_json(e),
        Expr::ForLoop(e) => for_loop_to_json(e),
        Expr::Group(e) => group_to_json(e),
        Expr::If(e) => if_to_json(e),
        Expr::Index(e) => index_to_json(e),
        Expr::Infer(e) => infer_to_json(e),
        Expr::Let(e) => let_to_json(e),
        Expr::Lit(e) => lit_expr_to_json(e),
        Expr::Loop(e) => loop_to_json(e),
        Expr::Macro(e) => macro_to_json(e),
        Expr::Match(e) => match_to_json(e),
        Expr::MethodCall(e) => method_call_to_json(e),
        Expr::Paren(e) => paren_to_json(e),
        Expr::Path(e) => path_expr_to_json(e),
        Expr::Range(e) => range_to_json(e),
        Expr::RawAddr(e) => raw_addr_to_json(e),
        Expr::Reference(e) => reference_to_json(e),
        Expr::Repeat(e) => repeat_to_json(e),
        Expr::Return(e) => return_to_json(e),
        Expr::Struct(e) => struct_to_json(e),
        Expr::Try(e) => try_to_json(e),
        Expr::TryBlock(e) => try_block_to_json(e),
        Expr::Tuple(e) => tuple_to_json(e),
        Expr::Unary(e) => unary_to_json(e),
        Expr::Unsafe(e) => unsafe_to_json(e),
        Expr::Verbatim(ts) => json!({
            "kind": "Verbatim",
            "tokens": ts.to_string()
        }),
        Expr::While(e) => while_to_json(e),
        Expr::Yield(e) => yield_to_json(e),
        // syn::Expr is #[non_exhaustive], so we must handle unknown variants.
        // This uses ToTokens to produce a stable representation for any future variants.
        #[allow(unreachable_patterns)]
        _ => json!({
            "kind": "Unknown",
            "tokens": expr.to_token_stream().to_string()
        }),
    }
}

// Helper functions for converting types to strings

/// Convert a syn::Type to a string representation.
pub fn type_to_string(ty: &Type) -> String {
    ty.to_token_stream().to_string()
}

/// Convert a syn::Pat to a string representation.
pub fn pat_to_string(pat: &Pat) -> String {
    pat.to_token_stream().to_string()
}

/// Convert a syn::Path to a string representation.
pub fn path_to_string(path: &syn::Path) -> String {
    path.to_token_stream().to_string()
}

/// Convert a syn::Lit to a serde_json::Value.
pub fn lit_to_json(lit: &Lit) -> Value {
    match lit {
        Lit::Str(s) => json!({
            "kind": "Str",
            "value": s.value(),
            "suffix": s.suffix()
        }),
        Lit::ByteStr(bs) => json!({
            "kind": "ByteStr",
            "value": bs.value(),
            "suffix": bs.suffix()
        }),
        Lit::CStr(cs) => json!({
            "kind": "CStr",
            "value": cs.value().to_string_lossy(),
            "suffix": cs.suffix()
        }),
        Lit::Byte(b) => json!({
            "kind": "Byte",
            "value": b.value(),
            "suffix": b.suffix()
        }),
        Lit::Char(c) => json!({
            "kind": "Char",
            "value": c.value().to_string(),
            "suffix": c.suffix()
        }),
        Lit::Int(i) => json!({
            "kind": "Int",
            "value": i.base10_digits(),
            "suffix": i.suffix()
        }),
        Lit::Float(f) => json!({
            "kind": "Float",
            "value": f.base10_digits(),
            "suffix": f.suffix()
        }),
        Lit::Bool(b) => json!({
            "kind": "Bool",
            "value": b.value()
        }),
        Lit::Verbatim(v) => json!({
            "kind": "Verbatim",
            "tokens": v.to_string()
        }),
        // syn::Lit is #[non_exhaustive], handle future variants
        #[allow(unreachable_patterns)]
        _ => json!({
            "kind": "Unknown",
            "tokens": lit.to_token_stream().to_string()
        }),
    }
}

/// Convert a Label to a string.
fn label_to_string(label: &Label) -> String {
    label.name.ident.to_string()
}

/// Convert an optional Label to a Value.
fn opt_label_to_json(label: &Option<Label>) -> Value {
    match label {
        Some(l) => json!(label_to_string(l)),
        None => Value::Null,
    }
}

/// Convert a Member to a Value.
fn member_to_json(member: &Member) -> Value {
    match member {
        Member::Named(ident) => json!({
            "kind": "Named",
            "name": ident.to_string()
        }),
        Member::Unnamed(Index { index, .. }) => json!({
            "kind": "Unnamed",
            "index": index
        }),
    }
}

/// Convert a BinOp to a Value.
fn binop_to_json(op: &BinOp) -> Value {
    let op_str = match op {
        BinOp::Add(_) => "+",
        BinOp::Sub(_) => "-",
        BinOp::Mul(_) => "*",
        BinOp::Div(_) => "/",
        BinOp::Rem(_) => "%",
        BinOp::And(_) => "&&",
        BinOp::Or(_) => "||",
        BinOp::BitXor(_) => "^",
        BinOp::BitAnd(_) => "&",
        BinOp::BitOr(_) => "|",
        BinOp::Shl(_) => "<<",
        BinOp::Shr(_) => ">>",
        BinOp::Eq(_) => "==",
        BinOp::Lt(_) => "<",
        BinOp::Le(_) => "<=",
        BinOp::Ne(_) => "!=",
        BinOp::Ge(_) => ">=",
        BinOp::Gt(_) => ">",
        BinOp::AddAssign(_) => "+=",
        BinOp::SubAssign(_) => "-=",
        BinOp::MulAssign(_) => "*=",
        BinOp::DivAssign(_) => "/=",
        BinOp::RemAssign(_) => "%=",
        BinOp::BitXorAssign(_) => "^=",
        BinOp::BitAndAssign(_) => "&=",
        BinOp::BitOrAssign(_) => "|=",
        BinOp::ShlAssign(_) => "<<=",
        BinOp::ShrAssign(_) => ">>=",
        // syn::BinOp is #[non_exhaustive], handle future variants
        #[allow(unreachable_patterns)]
        _ => return json!(op.to_token_stream().to_string()),
    };
    json!(op_str)
}

/// Convert a UnOp to a Value.
fn unop_to_json(op: &UnOp) -> Value {
    let op_str = match op {
        UnOp::Deref(_) => "*",
        UnOp::Not(_) => "!",
        UnOp::Neg(_) => "-",
        // syn::UnOp is #[non_exhaustive], handle future variants
        #[allow(unreachable_patterns)]
        _ => return json!(op.to_token_stream().to_string()),
    };
    json!(op_str)
}

/// Convert attributes to JSON.
fn attrs_to_json(attrs: &[syn::Attribute]) -> Value {
    json!(attrs
        .iter()
        .map(|a| a.to_token_stream().to_string())
        .collect::<Vec<_>>())
}

/// Convert an Arm to a Value.
fn arm_to_json(arm: &Arm) -> Value {
    json!({
        "attrs": attrs_to_json(&arm.attrs),
        "pat": pat_to_string(&arm.pat),
        "guard": arm.guard.as_ref().map(|(_, expr)| expr_to_json(expr)),
        "body": expr_to_json(&arm.body)
    })
}

/// Convert a FieldValue to a Value.
fn field_value_to_json(fv: &FieldValue) -> Value {
    json!({
        "attrs": attrs_to_json(&fv.attrs),
        "member": member_to_json(&fv.member),
        "expr": expr_to_json(&fv.expr)
    })
}

/// Convert RangeLimits to a Value.
fn range_limits_to_json(limits: &RangeLimits) -> Value {
    match limits {
        RangeLimits::HalfOpen(_) => json!("HalfOpen"),
        RangeLimits::Closed(_) => json!("Closed"),
    }
}

// Individual expr variant converters

fn array_to_json(e: &ExprArray) -> Value {
    json!({
        "kind": "Array",
        "attrs": attrs_to_json(&e.attrs),
        "elems": e.elems.iter().map(expr_to_json).collect::<Vec<_>>()
    })
}

fn assign_to_json(e: &ExprAssign) -> Value {
    json!({
        "kind": "Assign",
        "attrs": attrs_to_json(&e.attrs),
        "left": expr_to_json(&e.left),
        "right": expr_to_json(&e.right)
    })
}

fn async_to_json(e: &ExprAsync) -> Value {
    json!({
        "kind": "Async",
        "attrs": attrs_to_json(&e.attrs),
        "capture": e.capture.is_some(),
        "block": e.block.to_token_stream().to_string()
    })
}

fn await_to_json(e: &ExprAwait) -> Value {
    json!({
        "kind": "Await",
        "attrs": attrs_to_json(&e.attrs),
        "base": expr_to_json(&e.base)
    })
}

fn binary_to_json(e: &ExprBinary) -> Value {
    json!({
        "kind": "Binary",
        "attrs": attrs_to_json(&e.attrs),
        "left": expr_to_json(&e.left),
        "op": binop_to_json(&e.op),
        "right": expr_to_json(&e.right)
    })
}

fn block_to_json(e: &ExprBlock) -> Value {
    json!({
        "kind": "Block",
        "attrs": attrs_to_json(&e.attrs),
        "label": opt_label_to_json(&e.label),
        "block": e.block.to_token_stream().to_string()
    })
}

fn break_to_json(e: &ExprBreak) -> Value {
    json!({
        "kind": "Break",
        "attrs": attrs_to_json(&e.attrs),
        "label": e.label.as_ref().map(|l| l.ident.to_string()),
        "expr": e.expr.as_ref().map(|expr| expr_to_json(expr))
    })
}

fn call_to_json(e: &ExprCall) -> Value {
    json!({
        "kind": "Call",
        "attrs": attrs_to_json(&e.attrs),
        "func": expr_to_json(&e.func),
        "args": e.args.iter().map(expr_to_json).collect::<Vec<_>>()
    })
}

fn cast_to_json(e: &ExprCast) -> Value {
    json!({
        "kind": "Cast",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr),
        "ty": type_to_string(&e.ty)
    })
}

fn closure_to_json(e: &ExprClosure) -> Value {
    json!({
        "kind": "Closure",
        "attrs": attrs_to_json(&e.attrs),
        "lifetimes": e.lifetimes.as_ref().map(|l| l.to_token_stream().to_string()),
        "constness": e.constness.is_some(),
        "movability": e.movability.is_some(),
        "asyncness": e.asyncness.is_some(),
        "capture": e.capture.is_some(),
        "inputs": e.inputs.iter().map(pat_to_string).collect::<Vec<_>>(),
        "output": e.output.to_token_stream().to_string(),
        "body": expr_to_json(&e.body)
    })
}

fn const_to_json(e: &ExprConst) -> Value {
    json!({
        "kind": "Const",
        "attrs": attrs_to_json(&e.attrs),
        "block": e.block.to_token_stream().to_string()
    })
}

fn continue_to_json(e: &ExprContinue) -> Value {
    json!({
        "kind": "Continue",
        "attrs": attrs_to_json(&e.attrs),
        "label": e.label.as_ref().map(|l| l.ident.to_string())
    })
}

fn field_to_json(e: &ExprField) -> Value {
    json!({
        "kind": "Field",
        "attrs": attrs_to_json(&e.attrs),
        "base": expr_to_json(&e.base),
        "member": member_to_json(&e.member)
    })
}

fn for_loop_to_json(e: &ExprForLoop) -> Value {
    json!({
        "kind": "ForLoop",
        "attrs": attrs_to_json(&e.attrs),
        "label": opt_label_to_json(&e.label),
        "pat": pat_to_string(&e.pat),
        "expr": expr_to_json(&e.expr),
        "body": e.body.to_token_stream().to_string()
    })
}

fn group_to_json(e: &ExprGroup) -> Value {
    json!({
        "kind": "Group",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr)
    })
}

fn if_to_json(e: &ExprIf) -> Value {
    json!({
        "kind": "If",
        "attrs": attrs_to_json(&e.attrs),
        "cond": expr_to_json(&e.cond),
        "then_branch": e.then_branch.to_token_stream().to_string(),
        "else_branch": e.else_branch.as_ref().map(|(_, expr)| expr_to_json(expr))
    })
}

fn index_to_json(e: &ExprIndex) -> Value {
    json!({
        "kind": "Index",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr),
        "index": expr_to_json(&e.index)
    })
}

fn infer_to_json(e: &ExprInfer) -> Value {
    json!({
        "kind": "Infer",
        "attrs": attrs_to_json(&e.attrs)
    })
}

fn let_to_json(e: &ExprLet) -> Value {
    json!({
        "kind": "Let",
        "attrs": attrs_to_json(&e.attrs),
        "pat": pat_to_string(&e.pat),
        "expr": expr_to_json(&e.expr)
    })
}

fn lit_expr_to_json(e: &ExprLit) -> Value {
    json!({
        "kind": "Lit",
        "attrs": attrs_to_json(&e.attrs),
        "lit": lit_to_json(&e.lit)
    })
}

fn loop_to_json(e: &ExprLoop) -> Value {
    json!({
        "kind": "Loop",
        "attrs": attrs_to_json(&e.attrs),
        "label": opt_label_to_json(&e.label),
        "body": e.body.to_token_stream().to_string()
    })
}

fn macro_to_json(e: &ExprMacro) -> Value {
    json!({
        "kind": "Macro",
        "attrs": attrs_to_json(&e.attrs),
        "mac": e.mac.to_token_stream().to_string()
    })
}

fn match_to_json(e: &ExprMatch) -> Value {
    json!({
        "kind": "Match",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr),
        "arms": e.arms.iter().map(arm_to_json).collect::<Vec<_>>()
    })
}

fn method_call_to_json(e: &ExprMethodCall) -> Value {
    json!({
        "kind": "MethodCall",
        "attrs": attrs_to_json(&e.attrs),
        "receiver": expr_to_json(&e.receiver),
        "method": e.method.to_string(),
        "turbofish": e.turbofish.as_ref().map(|t| t.to_token_stream().to_string()),
        "args": e.args.iter().map(expr_to_json).collect::<Vec<_>>()
    })
}

fn paren_to_json(e: &ExprParen) -> Value {
    json!({
        "kind": "Paren",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr)
    })
}

fn path_expr_to_json(e: &ExprPath) -> Value {
    json!({
        "kind": "Path",
        "attrs": attrs_to_json(&e.attrs),
        "qself": e.qself.as_ref().map(|q| type_to_string(&q.ty)),
        "path": path_to_string(&e.path)
    })
}

fn range_to_json(e: &ExprRange) -> Value {
    json!({
        "kind": "Range",
        "attrs": attrs_to_json(&e.attrs),
        "start": e.start.as_ref().map(|expr| expr_to_json(expr)),
        "limits": range_limits_to_json(&e.limits),
        "end": e.end.as_ref().map(|expr| expr_to_json(expr))
    })
}

fn raw_addr_to_json(e: &ExprRawAddr) -> Value {
    let is_mut = matches!(e.mutability, PointerMutability::Mut(_));
    json!({
        "kind": "RawAddr",
        "attrs": attrs_to_json(&e.attrs),
        "mutability": is_mut,
        "expr": expr_to_json(&e.expr)
    })
}

fn reference_to_json(e: &ExprReference) -> Value {
    json!({
        "kind": "Reference",
        "attrs": attrs_to_json(&e.attrs),
        "mutability": e.mutability.is_some(),
        "expr": expr_to_json(&e.expr)
    })
}

fn repeat_to_json(e: &ExprRepeat) -> Value {
    json!({
        "kind": "Repeat",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr),
        "len": expr_to_json(&e.len)
    })
}

fn return_to_json(e: &ExprReturn) -> Value {
    json!({
        "kind": "Return",
        "attrs": attrs_to_json(&e.attrs),
        "expr": e.expr.as_ref().map(|expr| expr_to_json(expr))
    })
}

fn struct_to_json(e: &ExprStruct) -> Value {
    json!({
        "kind": "Struct",
        "attrs": attrs_to_json(&e.attrs),
        "qself": e.qself.as_ref().map(|q| type_to_string(&q.ty)),
        "path": path_to_string(&e.path),
        "fields": e.fields.iter().map(field_value_to_json).collect::<Vec<_>>(),
        "dot2_token": e.dot2_token.is_some(),
        "rest": e.rest.as_ref().map(|expr| expr_to_json(expr))
    })
}

fn try_to_json(e: &ExprTry) -> Value {
    json!({
        "kind": "Try",
        "attrs": attrs_to_json(&e.attrs),
        "expr": expr_to_json(&e.expr)
    })
}

fn try_block_to_json(e: &ExprTryBlock) -> Value {
    json!({
        "kind": "TryBlock",
        "attrs": attrs_to_json(&e.attrs),
        "block": e.block.to_token_stream().to_string()
    })
}

fn tuple_to_json(e: &ExprTuple) -> Value {
    json!({
        "kind": "Tuple",
        "attrs": attrs_to_json(&e.attrs),
        "elems": e.elems.iter().map(expr_to_json).collect::<Vec<_>>()
    })
}

fn unary_to_json(e: &ExprUnary) -> Value {
    json!({
        "kind": "Unary",
        "attrs": attrs_to_json(&e.attrs),
        "op": unop_to_json(&e.op),
        "expr": expr_to_json(&e.expr)
    })
}

fn unsafe_to_json(e: &ExprUnsafe) -> Value {
    json!({
        "kind": "Unsafe",
        "attrs": attrs_to_json(&e.attrs),
        "block": e.block.to_token_stream().to_string()
    })
}

fn while_to_json(e: &ExprWhile) -> Value {
    json!({
        "kind": "While",
        "attrs": attrs_to_json(&e.attrs),
        "label": opt_label_to_json(&e.label),
        "cond": expr_to_json(&e.cond),
        "body": e.body.to_token_stream().to_string()
    })
}

fn yield_to_json(e: &ExprYield) -> Value {
    json!({
        "kind": "Yield",
        "attrs": attrs_to_json(&e.attrs),
        "expr": e.expr.as_ref().map(|expr| expr_to_json(expr))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    fn parse_expr(code: &str) -> Expr {
        syn::parse_str(code).expect("Failed to parse expression")
    }

    #[test]
    fn test_literal_int() {
        let expr = parse_expr("42");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Lit");
        assert_eq!(json["lit"]["kind"], "Int");
        assert_eq!(json["lit"]["value"], "42");
    }

    #[test]
    fn test_literal_string() {
        let expr = parse_expr(r#""hello""#);
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Lit");
        assert_eq!(json["lit"]["kind"], "Str");
        assert_eq!(json["lit"]["value"], "hello");
    }

    #[test]
    fn test_literal_bool() {
        let expr = parse_expr("true");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Lit");
        assert_eq!(json["lit"]["kind"], "Bool");
        assert_eq!(json["lit"]["value"], true);
    }

    #[test]
    fn test_literal_float() {
        let expr = parse_expr("3.14");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Lit");
        assert_eq!(json["lit"]["kind"], "Float");
        assert_eq!(json["lit"]["value"], "3.14");
    }

    #[test]
    fn test_literal_char() {
        let expr = parse_expr("'a'");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Lit");
        assert_eq!(json["lit"]["kind"], "Char");
        assert_eq!(json["lit"]["value"], "a");
    }

    #[test]
    fn test_binary_add() {
        let expr = parse_expr("1 + 2");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Binary");
        assert_eq!(json["op"], "+");
        assert_eq!(json["left"]["kind"], "Lit");
        assert_eq!(json["right"]["kind"], "Lit");
    }

    #[test]
    fn test_binary_mul() {
        let expr = parse_expr("a * b");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Binary");
        assert_eq!(json["op"], "*");
    }

    #[test]
    fn test_binary_comparison() {
        let expr = parse_expr("x < y");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Binary");
        assert_eq!(json["op"], "<");
    }

    #[test]
    fn test_binary_logical() {
        let expr = parse_expr("a && b");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Binary");
        assert_eq!(json["op"], "&&");
    }

    #[test]
    fn test_unary_neg() {
        let expr = parse_expr("-x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Unary");
        assert_eq!(json["op"], "-");
    }

    #[test]
    fn test_unary_not() {
        let expr = parse_expr("!flag");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Unary");
        assert_eq!(json["op"], "!");
    }

    #[test]
    fn test_unary_deref() {
        let expr = parse_expr("*ptr");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Unary");
        assert_eq!(json["op"], "*");
    }

    #[test]
    fn test_path() {
        let expr = parse_expr("std::collections::HashMap");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Path");
        assert!(json["path"].as_str().unwrap().contains("HashMap"));
    }

    #[test]
    fn test_call() {
        let expr = parse_expr("foo(1, 2, 3)");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Call");
        assert_eq!(json["args"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_method_call() {
        let expr = parse_expr("obj.method(arg)");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "MethodCall");
        assert_eq!(json["method"], "method");
        assert_eq!(json["args"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_method_call_with_turbofish() {
        let expr = parse_expr("vec.iter::<i32>()");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "MethodCall");
        assert_eq!(json["method"], "iter");
        assert!(json["turbofish"].is_string());
    }

    #[test]
    fn test_array() {
        let expr = parse_expr("[1, 2, 3]");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Array");
        assert_eq!(json["elems"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_tuple() {
        let expr = parse_expr("(1, \"hello\", true)");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Tuple");
        assert_eq!(json["elems"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_index() {
        let expr = parse_expr("arr[0]");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Index");
    }

    #[test]
    fn test_field_named() {
        let expr = parse_expr("obj.field");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Field");
        assert_eq!(json["member"]["kind"], "Named");
        assert_eq!(json["member"]["name"], "field");
    }

    #[test]
    fn test_field_tuple() {
        let expr = parse_expr("tuple.0");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Field");
        assert_eq!(json["member"]["kind"], "Unnamed");
        assert_eq!(json["member"]["index"], 0);
    }

    #[test]
    fn test_reference() {
        let expr = parse_expr("&x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Reference");
        assert_eq!(json["mutability"], false);
    }

    #[test]
    fn test_reference_mut() {
        let expr = parse_expr("&mut x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Reference");
        assert_eq!(json["mutability"], true);
    }

    #[test]
    fn test_cast() {
        let expr = parse_expr("x as i32");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Cast");
        assert!(json["ty"].as_str().unwrap().contains("i32"));
    }

    #[test]
    fn test_assign() {
        let expr = parse_expr("x = 5");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Assign");
    }

    #[test]
    fn test_paren() {
        let expr = parse_expr("(a + b)");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Paren");
        assert_eq!(json["expr"]["kind"], "Binary");
    }

    #[test]
    fn test_if_simple() {
        let expr = parse_expr("if x { 1 } else { 2 }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "If");
        assert!(json["else_branch"].is_object());
    }

    #[test]
    fn test_if_without_else() {
        let expr = parse_expr("if x { 1 }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "If");
        assert!(json["else_branch"].is_null());
    }

    #[test]
    fn test_match() {
        let expr = parse_expr("match x { 1 => true, _ => false }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Match");
        assert_eq!(json["arms"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_match_with_guard() {
        let expr = parse_expr("match x { n if n > 0 => true, _ => false }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Match");
        let arms = json["arms"].as_array().unwrap();
        assert!(arms[0]["guard"].is_object());
    }

    #[test]
    fn test_closure_simple() {
        let expr = parse_expr("|x| x + 1");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Closure");
        assert_eq!(json["inputs"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_closure_move() {
        let expr = parse_expr("move |x| x + 1");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Closure");
        assert_eq!(json["capture"], true);
    }

    #[test]
    fn test_closure_async() {
        let expr = parse_expr("async |x| x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Closure");
        assert_eq!(json["asyncness"], true);
    }

    #[test]
    fn test_block() {
        let expr = parse_expr("{ let x = 1; x }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Block");
    }

    #[test]
    fn test_block_labeled() {
        let expr = parse_expr("'label: { break 'label 1 }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Block");
        assert_eq!(json["label"], "label");
    }

    #[test]
    fn test_loop() {
        let expr = parse_expr("loop { break }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Loop");
    }

    #[test]
    fn test_loop_labeled() {
        let expr = parse_expr("'outer: loop { break 'outer }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Loop");
        assert_eq!(json["label"], "outer");
    }

    #[test]
    fn test_while() {
        let expr = parse_expr("while x { y }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "While");
    }

    #[test]
    fn test_for_loop() {
        let expr = parse_expr("for x in items { process(x) }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "ForLoop");
        assert!(json["pat"].as_str().unwrap().contains("x"));
    }

    #[test]
    fn test_break() {
        let expr = parse_expr("break");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Break");
        assert!(json["expr"].is_null());
    }

    #[test]
    fn test_break_with_value() {
        let expr = parse_expr("break 42");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Break");
        assert!(json["expr"].is_object());
    }

    #[test]
    fn test_continue() {
        let expr = parse_expr("continue");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Continue");
    }

    #[test]
    fn test_continue_labeled() {
        let expr = parse_expr("continue 'outer");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Continue");
        assert_eq!(json["label"], "outer");
    }

    #[test]
    fn test_return() {
        let expr = parse_expr("return");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Return");
        assert!(json["expr"].is_null());
    }

    #[test]
    fn test_return_with_value() {
        let expr = parse_expr("return 42");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Return");
        assert!(json["expr"].is_object());
    }

    #[test]
    fn test_range_full() {
        let expr = parse_expr("..");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Range");
        assert!(json["start"].is_null());
        assert!(json["end"].is_null());
    }

    #[test]
    fn test_range_from() {
        let expr = parse_expr("0..");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Range");
        assert!(json["start"].is_object());
        assert!(json["end"].is_null());
    }

    #[test]
    fn test_range_to() {
        let expr = parse_expr("..10");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Range");
        assert!(json["start"].is_null());
        assert!(json["end"].is_object());
    }

    #[test]
    fn test_range_inclusive() {
        let expr = parse_expr("0..=10");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Range");
        assert_eq!(json["limits"], "Closed");
    }

    #[test]
    fn test_struct_init() {
        let expr = parse_expr("Point { x: 1, y: 2 }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Struct");
        assert_eq!(json["fields"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_struct_init_with_rest() {
        let expr = parse_expr("Point { x: 1, ..other }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Struct");
        assert_eq!(json["dot2_token"], true);
        assert!(json["rest"].is_object());
    }

    #[test]
    fn test_repeat() {
        let expr = parse_expr("[0; 10]");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Repeat");
    }

    #[test]
    fn test_try() {
        let expr = parse_expr("result?");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Try");
    }

    #[test]
    fn test_async_block() {
        let expr = parse_expr("async { foo().await }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Async");
    }

    #[test]
    fn test_async_move() {
        let expr = parse_expr("async move { x }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Async");
        assert_eq!(json["capture"], true);
    }

    #[test]
    fn test_await() {
        let expr = parse_expr("future.await");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Await");
    }

    #[test]
    fn test_unsafe_block() {
        let expr = parse_expr("unsafe { dangerous() }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Unsafe");
    }

    #[test]
    fn test_let_expr() {
        let expr = parse_expr("let Some(x) = opt");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Let");
    }

    #[test]
    fn test_macro() {
        let expr = parse_expr("println!(\"hello\")");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Macro");
    }

    #[test]
    fn test_const_block() {
        let expr = parse_expr("const { 1 + 1 }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Const");
    }

    #[test]
    fn test_infer() {
        let expr = parse_expr("_");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Infer");
    }

    #[test]
    fn test_group() {
        // Group expressions are typically created by macro expansion
        // We test through parse_quote which can create them
        let expr: Expr = parse_quote!(#[allow(unused)] { 1 });
        // This will be a Block with attrs
        let json = expr_to_json(&expr);
        assert!(json["kind"] == "Block");
    }

    #[test]
    fn test_complex_nested() {
        let expr = parse_expr("if x > 0 { Some(x * 2) } else { None }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "If");
        assert!(json["cond"]["kind"] == "Binary");
    }

    #[test]
    fn test_chained_method_calls() {
        let expr = parse_expr("vec.iter().map(|x| x * 2).collect()");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "MethodCall");
        assert_eq!(json["method"], "collect");
    }

    #[test]
    fn test_binary_assign_ops() {
        let test_cases = [
            ("x += 1", "+="),
            ("x -= 1", "-="),
            ("x *= 2", "*="),
            ("x /= 2", "/="),
            ("x %= 3", "%="),
            ("x &= 1", "&="),
            ("x |= 1", "|="),
            ("x ^= 1", "^="),
            ("x <<= 1", "<<="),
            ("x >>= 1", ">>="),
        ];

        for (code, expected_op) in test_cases {
            let expr = parse_expr(code);
            let json = expr_to_json(&expr);
            assert_eq!(json["kind"], "Binary", "Failed for: {}", code);
            assert_eq!(json["op"], expected_op, "Failed for: {}", code);
        }
    }

    #[test]
    fn test_all_comparison_ops() {
        let test_cases = [
            ("a == b", "=="),
            ("a != b", "!="),
            ("a < b", "<"),
            ("a <= b", "<="),
            ("a > b", ">"),
            ("a >= b", ">="),
        ];

        for (code, expected_op) in test_cases {
            let expr = parse_expr(code);
            let json = expr_to_json(&expr);
            assert_eq!(json["op"], expected_op, "Failed for: {}", code);
        }
    }

    #[test]
    fn test_bitwise_ops() {
        let test_cases = [
            ("a & b", "&"),
            ("a | b", "|"),
            ("a ^ b", "^"),
            ("a << b", "<<"),
            ("a >> b", ">>"),
        ];

        for (code, expected_op) in test_cases {
            let expr = parse_expr(code);
            let json = expr_to_json(&expr);
            assert_eq!(json["op"], expected_op, "Failed for: {}", code);
        }
    }

    #[test]
    fn test_path_with_generics() {
        let expr = parse_expr("Vec::<i32>::new()");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Call");
    }

    #[test]
    fn test_struct_shorthand() {
        let expr = parse_expr("Point { x, y }");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "Struct");
        let fields = json["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_deeply_nested() {
        let expr = parse_expr("((((a + b))))");
        let json = expr_to_json(&expr);
        // Multiple levels of Paren
        let mut current = &json;
        for _ in 0..4 {
            assert_eq!(current["kind"], "Paren");
            current = &current["expr"];
        }
        assert_eq!(current["kind"], "Binary");
    }

    #[test]
    fn test_helper_functions() {
        // Test type_to_string
        let ty: Type = syn::parse_str("Vec<i32>").unwrap();
        let ty_str = type_to_string(&ty);
        assert!(ty_str.contains("Vec"));

        // Test pat_to_string via a let expression (Pat doesn't implement Parse directly)
        let expr: Expr = syn::parse_str("let Some(x) = opt").unwrap();
        if let Expr::Let(let_expr) = expr {
            let pat_str = pat_to_string(&let_expr.pat);
            assert!(pat_str.contains("Some"));
        } else {
            panic!("Expected Let expression");
        }

        // Test path_to_string
        let path: syn::Path = syn::parse_str("std::vec::Vec").unwrap();
        let path_str = path_to_string(&path);
        assert!(path_str.contains("Vec"));
    }

    #[test]
    fn test_lit_to_json_byte() {
        let lit: Lit = syn::parse_str("b'x'").unwrap();
        let json = lit_to_json(&lit);
        assert_eq!(json["kind"], "Byte");
    }

    #[test]
    fn test_lit_to_json_bytestr() {
        let lit: Lit = syn::parse_str(r#"b"hello""#).unwrap();
        let json = lit_to_json(&lit);
        assert_eq!(json["kind"], "ByteStr");
    }

    #[test]
    fn test_raw_addr() {
        let expr = parse_expr("&raw const x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "RawAddr");
        assert_eq!(json["mutability"], false);
    }

    #[test]
    fn test_raw_addr_mut() {
        let expr = parse_expr("&raw mut x");
        let json = expr_to_json(&expr);
        assert_eq!(json["kind"], "RawAddr");
        assert_eq!(json["mutability"], true);
    }
}
