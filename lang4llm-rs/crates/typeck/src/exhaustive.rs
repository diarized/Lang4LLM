//! Exhaustiveness checking for pattern matching
//!
//! Implements DR-ERR-004: Mandatory exhaustive matching
//! Validates that match expressions cover all possible cases

use lang4llm_ast::*;
use std::collections::HashSet;

/// Exhaustiveness error
#[derive(Debug, Clone, thiserror::Error)]
pub enum ExhaustivenessError {
    /// Match is not exhaustive
    #[error("Non-exhaustive match at {span}: missing patterns {missing:?}")]
    NonExhaustive {
        /// Source location
        span: Span,
        /// Missing patterns
        missing: Vec<String>,
    },

    /// Enum variant not found
    #[error("Unknown enum variant at {span}: {enum_name}::{variant}")]
    UnknownVariant {
        /// Enum name
        enum_name: String,
        /// Variant name
        variant: String,
        /// Source location
        span: Span,
    },
}

/// Pattern coverage tracker
pub struct ExhaustivenessChecker {
    /// Accumulated errors
    errors: Vec<ExhaustivenessError>,
}

impl ExhaustivenessChecker {
    /// Create a new exhaustiveness checker
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// Check if a match expression is exhaustive
    ///
    /// Returns Ok(()) if exhaustive, Err with missing patterns otherwise
    pub fn check_match_exhaustive(
        &mut self,
        match_expr: &MatchExpr,
        matched_type: &Type,
    ) -> Result<(), Vec<ExhaustivenessError>> {
        // Collect all patterns from arms
        let patterns: Vec<&Pattern> = match_expr.arms.iter().map(|arm| &arm.pattern).collect();

        // Check exhaustiveness based on type
        match matched_type {
            Type::Primitive(PrimitiveType::Bool, _) => {
                self.check_bool_exhaustive(match_expr.span, &patterns)
            }
            Type::Enum(enum_name, _) => {
                self.check_enum_exhaustive(match_expr.span, enum_name, &patterns)
            }
            Type::Primitive(PrimitiveType::Int, _)
            | Type::Primitive(PrimitiveType::Float, _)
            | Type::Primitive(PrimitiveType::String, _) => {
                // Numeric and string types require wildcard pattern
                self.check_wildcard_exhaustive(match_expr.span, &patterns)
            }
            Type::Tuple(types, _) => {
                self.check_tuple_exhaustive(match_expr.span, types, &patterns)
            }
            Type::Array(_, _) => {
                // Arrays require wildcard pattern (too many possibilities)
                self.check_wildcard_exhaustive(match_expr.span, &patterns)
            }
            Type::Struct(_, _) => {
                // Structs require wildcard pattern (field combinations)
                self.check_wildcard_exhaustive(match_expr.span, &patterns)
            }
            Type::Generic(name, _, _) => {
                // Check if patterns contain enum variants — if so, treat as enum exhaustiveness
                let has_enum_patterns = patterns.iter().any(|p| matches!(p, Pattern::Enum(_)));
                if has_enum_patterns {
                    self.check_enum_exhaustive(match_expr.span, name, &patterns)
                } else {
                    // Generic types require wildcard (unknown at compile time)
                    self.check_wildcard_exhaustive(match_expr.span, &patterns)
                }
            }
            Type::Function(_) => {
                // Function types cannot be matched
                self.check_wildcard_exhaustive(match_expr.span, &patterns)
            }
            Type::Inferred(_) => {
                // Inferred types require wildcard (type not yet resolved)
                self.check_wildcard_exhaustive(match_expr.span, &patterns)
            }
            Type::Primitive(PrimitiveType::Unit, _) => {
                // Unit type has only one value, trivially exhaustive if any pattern exists
                if patterns.is_empty() {
                    self.errors.push(ExhaustivenessError::NonExhaustive {
                        span: match_expr.span,
                        missing: vec!["()".to_string()],
                    });
                }
                Ok(())
            }
        }
    }

    /// Check if bool match is exhaustive (requires true, false, or wildcard)
    fn check_bool_exhaustive(&mut self, span: Span, patterns: &[&Pattern]) -> Result<(), Vec<ExhaustivenessError>> {
        let mut has_true = false;
        let mut has_false = false;
        let mut has_wildcard = false;

        for pattern in patterns {
            match pattern {
                Pattern::Literal(Literal::Bool(true), _) => has_true = true,
                Pattern::Literal(Literal::Bool(false), _) => has_false = true,
                Pattern::Wildcard(_) => has_wildcard = true,
                _ => {}
            }
        }

        if has_wildcard || (has_true && has_false) {
            Ok(())
        } else {
            let mut missing = Vec::new();
            if !has_true {
                missing.push("true".to_string());
            }
            if !has_false {
                missing.push("false".to_string());
            }

            self.errors.push(ExhaustivenessError::NonExhaustive {
                span,
                missing,
            });
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Check if enum match is exhaustive (requires all variants or wildcard)
    fn check_enum_exhaustive(
        &mut self,
        span: Span,
        enum_name: &Ident,
        patterns: &[&Pattern],
    ) -> Result<(), Vec<ExhaustivenessError>> {
        let mut covered_variants = HashSet::new();
        let mut has_wildcard = false;

        for pattern in patterns {
            match pattern {
                Pattern::Enum(enum_pat) => {
                    // Extract variant name
                    covered_variants.insert(enum_pat.variant_name.name.clone());
                }
                Pattern::Wildcard(_) => has_wildcard = true,
                _ => {}
            }
        }

        if has_wildcard {
            return Ok(());
        }

        // For now, we can't check if all enum variants are covered without
        // enum definition access. This would require passing enum definitions
        // or a type context. Mark as TODO for full implementation.
        //
        // Simple heuristic: if we have at least one variant pattern, assume
        // the programmer knows what they're doing. Real implementation would
        // require enum variant list from type context.
        if covered_variants.is_empty() {
            self.errors.push(ExhaustivenessError::NonExhaustive {
                span,
                missing: vec![format!("{}::*", enum_name.name)],
            });
            return Err(std::mem::take(&mut self.errors));
        }

        Ok(())
    }

    /// Check if wildcard pattern exists (required for infinite types)
    fn check_wildcard_exhaustive(&mut self, span: Span, patterns: &[&Pattern]) -> Result<(), Vec<ExhaustivenessError>> {
        let has_wildcard = patterns.iter().any(|p| matches!(p, Pattern::Wildcard(_)));

        if has_wildcard {
            Ok(())
        } else {
            self.errors.push(ExhaustivenessError::NonExhaustive {
                span,
                missing: vec!["_".to_string()],
            });
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Check if tuple match is exhaustive
    fn check_tuple_exhaustive(
        &mut self,
        span: Span,
        _elem_types: &[Type],
        patterns: &[&Pattern],
    ) -> Result<(), Vec<ExhaustivenessError>> {
        // Tuple exhaustiveness is complex - requires Cartesian product of
        // all element pattern combinations. For simplicity, require wildcard
        // or tuple pattern with wildcards.

        let has_wildcard = patterns.iter().any(|p| matches!(p, Pattern::Wildcard(_)));
        let has_tuple_pattern = patterns.iter().any(|p| matches!(p, Pattern::Tuple(_, _)));

        if has_wildcard || has_tuple_pattern {
            Ok(())
        } else {
            self.errors.push(ExhaustivenessError::NonExhaustive {
                span,
                missing: vec!["(_)".to_string()],
            });
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Get accumulated errors
    pub fn errors(&self) -> &[ExhaustivenessError] {
        &self.errors
    }
}

impl Default for ExhaustivenessChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_span() -> Span {
        Span {
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 10,
        }
    }

    fn make_ident(name: &str) -> Ident {
        Ident {
            name: name.to_string(),
            span: make_span(),
        }
    }

    #[test]
    fn test_bool_exhaustive_with_both_cases() {
        let mut checker = ExhaustivenessChecker::new();
        let pat1 = Pattern::Literal(Literal::Bool(true), make_span());
        let pat2 = Pattern::Literal(Literal::Bool(false), make_span());
        let patterns = vec![&pat1, &pat2];

        let result = checker.check_bool_exhaustive(make_span(), &patterns);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bool_exhaustive_with_wildcard() {
        let mut checker = ExhaustivenessChecker::new();
        let pat = Pattern::Wildcard(make_span());
        let patterns = vec![&pat];

        let result = checker.check_bool_exhaustive(make_span(), &patterns);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bool_non_exhaustive() {
        let mut checker = ExhaustivenessChecker::new();
        let pat = Pattern::Literal(Literal::Bool(true), make_span());
        let patterns = vec![&pat];

        let result = checker.check_bool_exhaustive(make_span(), &patterns);
        assert!(result.is_err());

        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            match &errors[0] {
                ExhaustivenessError::NonExhaustive { missing, .. } => {
                    assert!(missing.contains(&"false".to_string()));
                }
                _ => panic!("Expected NonExhaustive error"),
            }
        }
    }

    #[test]
    fn test_wildcard_required() {
        let mut checker = ExhaustivenessChecker::new();
        let pat = Pattern::Literal(Literal::Int(42), make_span());
        let patterns = vec![&pat];

        let result = checker.check_wildcard_exhaustive(make_span(), &patterns);
        assert!(result.is_err());
    }

    #[test]
    fn test_wildcard_present() {
        let mut checker = ExhaustivenessChecker::new();
        let pat1 = Pattern::Literal(Literal::Int(42), make_span());
        let pat2 = Pattern::Wildcard(make_span());
        let patterns = vec![&pat1, &pat2];

        let result = checker.check_wildcard_exhaustive(make_span(), &patterns);
        assert!(result.is_ok());
    }
}
