//! Builtin function detection and translation.
//!
//! Lang4LLM builtins have no direct equivalents in Rust — they require
//! syntactic transformations (macro calls, receiver rewrites, casts).
//!
//! ## Full Stdlib Registry (29 functions)
//!
//! ### Tier 1 — Already Implemented
//! | Function | Rust Output |
//! |----------|-------------|
//! | `print(s)` | `println!("{}", s)` |
//! | `range(a, b)` | `a..b` (for-loop) |
//! | `push(arr, x)` | `arr.push(x)` |
//! | `len(x)` | `x.len()` |
//! | `sqrt(x)` | `(x).sqrt()` |
//! | `as_float(x)` | `(x as f64)` |
//! | `as_int(x)` | `format!("{}", x).parse::<i64>().unwrap_or(0)` |
//!
//! ### Tier 2 — Specified (pop, enumerate)
//! | Function | Rust Output |
//! |----------|-------------|
//! | `pop(arr)` | `arr.pop().unwrap()` |
//! | `enumerate(arr)` | `arr.into_iter().enumerate()` |
//!
//! ### Tier 3 — Math (abs, min, max, pow, floor, ceil, round)
//! | Function | Rust Output |
//! |----------|-------------|
//! | `abs(x)` | `(x).abs()` |
//! | `min(a, b)` | `std::cmp::min(a, b)` |
//! | `max(a, b)` | `std::cmp::max(a, b)` |
//! | `pow(b, e)` | `(b).pow(e as u32)` |
//! | `floor(x)` | `(x).floor()` |
//! | `ceil(x)` | `(x).ceil()` |
//! | `round(x)` | `(x).round()` |
//!
//! ### Tier 4 — String (to_string, contains, split, trim, replace, starts_with, ends_with, to_upper, to_lower, chars)
//! | Function | Rust Output |
//! |----------|-------------|
//! | `to_string(x)` | `format!("{}", x)` |
//! | `contains(s, sub)` | `s.contains(sub.as_str())` |
//! | `split(s, d)` | `s.split(d.as_str()).map(\|s\| s.to_string()).collect::<Vec<String>>()` |
//! | `trim(s)` | `s.trim().to_string()` |
//! | `replace(s, f, t)` | `s.replace(f.as_str(), t.as_str())` |
//! | `starts_with(s, p)` | `s.starts_with(p.as_str())` |
//! | `ends_with(s, suf)` | `s.ends_with(suf.as_str())` |
//! | `to_upper(s)` | `s.to_uppercase()` |
//! | `to_lower(s)` | `s.to_lowercase()` |
//! | `chars(s)` | `s.chars().map(\|c\| c.to_string()).collect::<Vec<String>>()` |
//!
//! ### Tier 5 — Array (sort, reverse, map, filter, is_empty, contains, concat, join, first)
//! | Function | Rust Output |
//! |----------|-------------|
//! | `sort(arr)` | `arr.sort()` |
//! | `reverse(arr)` | `arr.reverse()` |
//! | `map(arr, f)` | `arr.into_iter().map(f).collect::<Vec<_>>()` |
//! | `filter(arr, f)` | `arr.into_iter().filter(\|x\| f(x.clone())).collect::<Vec<_>>()` |
//! | `is_empty(arr)` | `arr.is_empty()` |
//! | `contains(arr, x)` | `arr.contains(&x)` |
//! | `concat(a, b)` | `{ let mut __v = a; __v.extend(b); __v }` |
//! | `join(arr, sep)` | `arr.join(sep.as_str())` |
//! | `first(arr)` | `arr.first().cloned().unwrap()` |

use lang4llm_ast::Expr;

use crate::context::CodegenContext;

/// Returns `true` if `name` is a Lang4LLM builtin function.
pub fn is_builtin(name: &str) -> bool {
    matches!(
        name,
        // Tier 1
        "print" | "range" | "push" | "len" | "sqrt" | "as_float" | "as_int"
        // Tier 2
        | "pop" | "enumerate"
        // Tier 3: Math
        | "abs" | "min" | "max" | "pow" | "floor" | "ceil" | "round"
        // Tier 4: String
        | "to_string" | "contains" | "split" | "trim" | "replace"
        | "starts_with" | "ends_with" | "to_upper" | "to_lower" | "chars"
        // Tier 5: Array
        | "sort" | "reverse" | "map" | "filter" | "is_empty"
        | "concat" | "join" | "first"
    )
}

/// Emit a builtin call, transforming it to its Rust equivalent.
///
/// Callers are responsible for ensuring `is_builtin(name)` is true.
pub fn emit_builtin_call(
    name: &str,
    args: &[Expr],
    ctx: &mut CodegenContext,
    emit_expr: &mut dyn FnMut(&Expr, &mut CodegenContext),
) {
    match name {
        // ── Tier 1 ──────────────────────────────────────────────────────────

        "print" => {
            // print(expr) → println!("{}", expr)
            if args.len() == 1 {
                ctx.write("println!(\"{}\", ");
                emit_expr(&args[0], ctx);
                ctx.write(")");
            } else {
                ctx.write("println!(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        ctx.write(", ");
                    }
                    emit_expr(arg, ctx);
                }
                ctx.write(")");
            }
        }
        "range" => {
            // range(a, b) → a..b (used in for-loop context)
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write("..");
                emit_expr(&args[1], ctx);
            }
        }
        "push" => {
            // push(arr, x) → arr.push(x)
            // Add .clone() for variable/field-access elements to avoid move-out errors.
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".push(");
                match &args[1] {
                    Expr::Variable(_, _) | Expr::FieldAccess(_) => {
                        emit_expr(&args[1], ctx);
                        ctx.write(".clone()");
                    }
                    _ => emit_expr(&args[1], ctx),
                }
                ctx.write(")");
            }
        }
        "len" => {
            // len(arr) → (arr.len() as i64)
            // Wrapping in parens prevents Rust from misinterpreting `arr.len() as i64 < x`
            // as a generic argument `i64<x>` rather than a comparison.
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(".len() as i64)");
            }
        }
        "sqrt" => {
            // sqrt(x) → (x).sqrt()
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").sqrt()");
            }
        }
        "as_float" => {
            // as_float(x) → format!("{}", x).parse::<f64>().unwrap_or(0.0)
            // Parse-based conversion works for both numeric types (i64) and String inputs.
            // A direct `as f64` cast fails for String arguments.
            if args.len() == 1 {
                ctx.write("{ let __sf = format!(\"{}\", ");
                emit_expr(&args[0], ctx);
                ctx.write("); __sf.parse::<f64>().unwrap_or(0.0) }");
            }
        }
        "as_int" => {
            // as_int(x) → { let __s = format!("{}", x); __s.parse::<i64>().unwrap_or(0) }
            if args.len() == 1 {
                ctx.write("{ let __s = format!(\"{}\", ");
                emit_expr(&args[0], ctx);
                ctx.write("); __s.parse::<i64>().unwrap_or(0) }");
            }
        }

        // ── Tier 2: Specified ────────────────────────────────────────────────

        "pop" => {
            // pop(arr) → arr.pop().unwrap()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".pop().unwrap()");
            }
        }
        "enumerate" => {
            // enumerate(arr) → arr.into_iter().enumerate()
            // (valid in for-loop iterator position)
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".into_iter().enumerate()");
            }
        }

        // ── Tier 3: Math ─────────────────────────────────────────────────────

        "abs" => {
            // abs(x) → (x).abs()  — works for both i64 and f64 via Rust trait dispatch
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").abs()");
            }
        }
        "min" => {
            // min(a, b) → std::cmp::min(a, b)
            if args.len() == 2 {
                ctx.write("std::cmp::min(");
                emit_expr(&args[0], ctx);
                ctx.write(", ");
                emit_expr(&args[1], ctx);
                ctx.write(")");
            }
        }
        "max" => {
            // max(a, b) → std::cmp::max(a, b)
            if args.len() == 2 {
                ctx.write("std::cmp::max(");
                emit_expr(&args[0], ctx);
                ctx.write(", ");
                emit_expr(&args[1], ctx);
                ctx.write(")");
            }
        }
        "pow" => {
            // pow(base, exp) → (base).pow(exp as u32)
            if args.len() == 2 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").pow(");
                emit_expr(&args[1], ctx);
                ctx.write(" as u32)");
            }
        }
        "floor" => {
            // floor(x) → (x).floor()
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").floor()");
            }
        }
        "ceil" => {
            // ceil(x) → (x).ceil()
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").ceil()");
            }
        }
        "round" => {
            // round(x) → (x).round()
            if args.len() == 1 {
                ctx.write("(");
                emit_expr(&args[0], ctx);
                ctx.write(").round()");
            }
        }

        // ── Tier 4: String ───────────────────────────────────────────────────

        "to_string" => {
            // to_string(x) → format!("{}", x)
            if args.len() == 1 {
                ctx.write("format!(\"{}\", ");
                emit_expr(&args[0], ctx);
                ctx.write(")");
            }
        }
        "contains" => {
            // Overloaded:
            //   contains(s: string, sub: string) → s.contains(sub.as_str())
            //   contains(arr: [T], x: T)         → arr.contains(&x)
            // Transpiler emits string version (as_str) for 2-string args,
            // reference version (&x) for array variant.
            // Since we have no type info here, we detect by inspecting arg structure:
            // always emit the general receiver.contains(&arg) pattern — valid for both.
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".contains(");
                // For string: need .as_str(); for array: need &item.
                // Emit reference form which is safe for arrays; string literals work too
                // because &String coerces to &str in Rust contains().
                ctx.write("&");
                emit_expr(&args[1], ctx);
                ctx.write(")");
            }
        }
        "split" => {
            // split(s, delim) → s.split(delim.as_str()).map(|s| s.to_string()).collect::<Vec<String>>()
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".split(");
                emit_expr(&args[1], ctx);
                ctx.write(".as_str()).map(|s| s.to_string()).collect::<Vec<String>>()");
            }
        }
        "trim" => {
            // trim(s) → s.trim().to_string()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".trim().to_string()");
            }
        }
        "replace" => {
            // replace(s, from, to) → s.replace(from.as_str(), to.as_str())
            if args.len() == 3 {
                emit_expr(&args[0], ctx);
                ctx.write(".replace(");
                emit_expr(&args[1], ctx);
                ctx.write(".as_str(), ");
                emit_expr(&args[2], ctx);
                ctx.write(".as_str())");
            }
        }
        "starts_with" => {
            // starts_with(s, prefix) → s.starts_with(prefix.as_str())
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".starts_with(");
                emit_expr(&args[1], ctx);
                ctx.write(".as_str())");
            }
        }
        "ends_with" => {
            // ends_with(s, suffix) → s.ends_with(suffix.as_str())
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".ends_with(");
                emit_expr(&args[1], ctx);
                ctx.write(".as_str())");
            }
        }
        "to_upper" => {
            // to_upper(s) → s.to_uppercase()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".to_uppercase()");
            }
        }
        "to_lower" => {
            // to_lower(s) → s.to_lowercase()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".to_lowercase()");
            }
        }
        "chars" => {
            // chars(s) → s.chars().map(|c| c.to_string()).collect::<Vec<String>>()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".chars().map(|c| c.to_string()).collect::<Vec<String>>()");
            }
        }

        // ── Tier 5: Array ────────────────────────────────────────────────────

        "sort" => {
            // sort(arr) → arr.sort_by(|a, b| a.partial_cmp(b).unwrap())
            // Using partial_cmp instead of sort() so that f64 arrays (which don't
            // implement Ord) are handled correctly alongside i64, String, bool.
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".sort_by(|a, b| a.partial_cmp(b).unwrap())");
            }
        }
        "reverse" => {
            // reverse(arr) → arr.reverse()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".reverse()");
            }
        }
        "map" => {
            // map(arr, f) → arr.into_iter().map(f).collect::<Vec<_>>()
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".into_iter().map(");
                emit_expr(&args[1], ctx);
                ctx.write(").collect::<Vec<_>>()");
            }
        }
        "filter" => {
            // filter(arr, f) → arr.into_iter().filter(|x| f(x.clone())).collect::<Vec<_>>()
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".into_iter().filter(|x| (");
                emit_expr(&args[1], ctx);
                ctx.write(")(x.clone())).collect::<Vec<_>>()");
            }
        }
        "is_empty" => {
            // is_empty(arr) → arr.is_empty()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".is_empty()");
            }
        }
        "concat" => {
            // concat(a, b) → { let mut __v = a; __v.extend(b); __v }
            if args.len() == 2 {
                ctx.write("{ let mut __v = ");
                emit_expr(&args[0], ctx);
                ctx.write("; __v.extend(");
                emit_expr(&args[1], ctx);
                ctx.write("); __v }");
            }
        }
        "join" => {
            // join(arr, sep) → arr.join(sep.as_str())
            if args.len() == 2 {
                emit_expr(&args[0], ctx);
                ctx.write(".join(");
                emit_expr(&args[1], ctx);
                ctx.write(".as_str())");
            }
        }
        "first" => {
            // first(arr) → arr.first().cloned().unwrap()
            if args.len() == 1 {
                emit_expr(&args[0], ctx);
                ctx.write(".first().cloned().unwrap()");
            }
        }

        _ => {
            // Unknown builtin — should not happen if is_builtin() was checked
            ctx.write(name);
            ctx.write("(");
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    ctx.write(", ");
                }
                emit_expr(arg, ctx);
            }
            ctx.write(")");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang4llm_ast::{Expr, Ident, Literal, Span};

    fn var(name: &str) -> Expr {
        Expr::Variable(
            Ident {
                name: name.to_string(),
                span: Span::dummy(),
            },
            Span::dummy(),
        )
    }

    fn int_lit(n: i64) -> Expr {
        Expr::Literal(Literal::Int(n), Span::dummy())
    }

    fn str_lit(s: &str) -> Expr {
        Expr::Literal(Literal::String(s.to_string()), Span::dummy())
    }

    fn float_lit(f: f64) -> Expr {
        Expr::Literal(Literal::Float(f), Span::dummy())
    }

    fn emit(name: &str, args: Vec<Expr>) -> String {
        let mut ctx = CodegenContext::new();
        let mut emit_expr_fn = crate::exprs::emit_expr;
        emit_builtin_call(name, &args, &mut ctx, &mut emit_expr_fn);
        ctx.finish()
    }

    // ── Tier 1 ──────────────────────────────────────────────────────────────

    #[test]
    fn test_is_builtin_known() {
        // Tier 1
        assert!(is_builtin("print"));
        assert!(is_builtin("range"));
        assert!(is_builtin("push"));
        assert!(is_builtin("len"));
        assert!(is_builtin("sqrt"));
        assert!(is_builtin("as_float"));
        assert!(is_builtin("as_int"));
        // Tier 2
        assert!(is_builtin("pop"));
        assert!(is_builtin("enumerate"));
        // Tier 3
        assert!(is_builtin("abs"));
        assert!(is_builtin("min"));
        assert!(is_builtin("max"));
        assert!(is_builtin("pow"));
        assert!(is_builtin("floor"));
        assert!(is_builtin("ceil"));
        assert!(is_builtin("round"));
        // Tier 4
        assert!(is_builtin("to_string"));
        assert!(is_builtin("contains"));
        assert!(is_builtin("split"));
        assert!(is_builtin("trim"));
        assert!(is_builtin("replace"));
        assert!(is_builtin("starts_with"));
        assert!(is_builtin("ends_with"));
        assert!(is_builtin("to_upper"));
        assert!(is_builtin("to_lower"));
        assert!(is_builtin("chars"));
        // Tier 5
        assert!(is_builtin("sort"));
        assert!(is_builtin("reverse"));
        assert!(is_builtin("map"));
        assert!(is_builtin("filter"));
        assert!(is_builtin("is_empty"));
        assert!(is_builtin("concat"));
        assert!(is_builtin("join"));
        assert!(is_builtin("first"));
    }

    #[test]
    fn test_is_builtin_unknown() {
        assert!(!is_builtin("println"));
        assert!(!is_builtin("foo"));
        assert!(!is_builtin("collect"));
        assert!(!is_builtin("unwrap"));
    }

    // ── Tier 2 ──────────────────────────────────────────────────────────────

    #[test]
    fn test_pop_builtin() {
        assert_eq!(emit("pop", vec![var("arr")]), "arr.pop().unwrap()");
    }

    #[test]
    fn test_enumerate_builtin() {
        assert_eq!(
            emit("enumerate", vec![var("arr")]),
            "arr.into_iter().enumerate()"
        );
    }

    // ── Tier 3: Math ─────────────────────────────────────────────────────────

    #[test]
    fn test_abs_builtin() {
        assert_eq!(emit("abs", vec![var("x")]), "(x).abs()");
    }

    #[test]
    fn test_min_builtin() {
        assert_eq!(
            emit("min", vec![var("a"), var("b")]),
            "std::cmp::min(a, b)"
        );
    }

    #[test]
    fn test_max_builtin() {
        assert_eq!(
            emit("max", vec![var("a"), var("b")]),
            "std::cmp::max(a, b)"
        );
    }

    #[test]
    fn test_pow_builtin() {
        // Int literals emit with _i64 suffix from the expression emitter
        assert_eq!(
            emit("pow", vec![int_lit(2), int_lit(10)]),
            "(2_i64).pow(10_i64 as u32)"
        );
    }

    #[test]
    fn test_floor_builtin() {
        assert_eq!(emit("floor", vec![float_lit(3.7)]), "(3.7).floor()");
    }

    #[test]
    fn test_ceil_builtin() {
        assert_eq!(emit("ceil", vec![float_lit(3.2)]), "(3.2).ceil()");
    }

    #[test]
    fn test_round_builtin() {
        assert_eq!(emit("round", vec![float_lit(3.5)]), "(3.5).round()");
    }

    // ── Tier 4: String ───────────────────────────────────────────────────────

    #[test]
    fn test_to_string_builtin() {
        // Int literals emit with _i64 suffix from the expression emitter
        assert_eq!(emit("to_string", vec![int_lit(42)]), "format!(\"{}\", 42_i64)");
    }

    #[test]
    fn test_contains_builtin() {
        // String literals are owned `.to_string()`; `&String` coerces to `&str` in Rust
        assert_eq!(
            emit("contains", vec![var("s"), str_lit("hello")]),
            r#"s.contains(&"hello".to_string())"#
        );
    }

    #[test]
    fn test_split_builtin() {
        // String literal emits `.to_string()`, then `.as_str()` converts back
        assert_eq!(
            emit("split", vec![var("s"), str_lit(",")]),
            r#"s.split(",".to_string().as_str()).map(|s| s.to_string()).collect::<Vec<String>>()"#
        );
    }

    #[test]
    fn test_trim_builtin() {
        assert_eq!(emit("trim", vec![var("s")]), "s.trim().to_string()");
    }

    #[test]
    fn test_replace_builtin() {
        assert_eq!(
            emit("replace", vec![var("s"), str_lit("old"), str_lit("new")]),
            r#"s.replace("old".to_string().as_str(), "new".to_string().as_str())"#
        );
    }

    #[test]
    fn test_starts_with_builtin() {
        assert_eq!(
            emit("starts_with", vec![var("s"), str_lit("http")]),
            r#"s.starts_with("http".to_string().as_str())"#
        );
    }

    #[test]
    fn test_ends_with_builtin() {
        assert_eq!(
            emit("ends_with", vec![var("s"), str_lit(".rs")]),
            r#"s.ends_with(".rs".to_string().as_str())"#
        );
    }

    #[test]
    fn test_to_upper_builtin() {
        assert_eq!(emit("to_upper", vec![var("name")]), "name.to_uppercase()");
    }

    #[test]
    fn test_to_lower_builtin() {
        assert_eq!(emit("to_lower", vec![var("name")]), "name.to_lowercase()");
    }

    #[test]
    fn test_chars_builtin() {
        assert_eq!(
            emit("chars", vec![var("s")]),
            "s.chars().map(|c| c.to_string()).collect::<Vec<String>>()"
        );
    }

    // ── Tier 5: Array ────────────────────────────────────────────────────────

    #[test]
    fn test_sort_builtin() {
        assert_eq!(emit("sort", vec![var("arr")]), "arr.sort_by(|a, b| a.partial_cmp(b).unwrap())");
    }

    #[test]
    fn test_reverse_builtin() {
        assert_eq!(emit("reverse", vec![var("arr")]), "arr.reverse()");
    }

    #[test]
    fn test_map_builtin() {
        assert_eq!(
            emit("map", vec![var("arr"), var("f")]),
            "arr.into_iter().map(f).collect::<Vec<_>>()"
        );
    }

    #[test]
    fn test_filter_builtin() {
        assert_eq!(
            emit("filter", vec![var("arr"), var("pred")]),
            "arr.into_iter().filter(|x| (pred)(x.clone())).collect::<Vec<_>>()"
        );
    }

    #[test]
    fn test_is_empty_builtin() {
        assert_eq!(emit("is_empty", vec![var("list")]), "list.is_empty()");
    }

    #[test]
    fn test_concat_builtin() {
        assert_eq!(
            emit("concat", vec![var("a"), var("b")]),
            "{ let mut __v = a; __v.extend(b); __v }"
        );
    }

    #[test]
    fn test_join_builtin() {
        assert_eq!(
            emit("join", vec![var("words"), str_lit(", ")]),
            r#"words.join(", ".to_string().as_str())"#
        );
    }

    #[test]
    fn test_first_builtin() {
        assert_eq!(
            emit("first", vec![var("arr")]),
            "arr.first().cloned().unwrap()"
        );
    }
}
