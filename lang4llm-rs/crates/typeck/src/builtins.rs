//! Builtin functions and types for Lang4LLM
//!
//! This module defines the standard library functions that are available
//! in all Lang4LLM programs without explicit imports.

use crate::symtab::{Symbol, SymbolKind, SymbolTable};
use lang4llm_ast::*;

/// Initialize the symbol table with builtin functions
pub fn initialize_builtins(symtab: &mut SymbolTable) {
    // print(string) -> unit
    // Prints a string to standard output
    let print_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::String, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::Unit, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "print".to_string(),
            ty: Some(print_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'print'");

    // range(int, int) -> Iterator<int>
    // Creates an iterator over a range of integers [start, end)
    // Note: For now, we'll represent Iterator<int> as a generic type
    // In a full implementation, this would be a proper Iterator trait
    let range_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
        ],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "Iterator".to_string(),
                span: Span::dummy(),
            },
            Some(GenericArgs {
                args: vec![Type::Primitive(PrimitiveType::Int, Span::dummy())],
                span: Span::dummy(),
            }),
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "range".to_string(),
            ty: Some(range_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'range'");

    // len(any) -> int
    // Works on arrays, strings, and collections
    // Uses generic T parameter to accept any array type
    let len_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "len".to_string(),
            ty: Some(len_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'len'");

    // push(array, element) -> unit
    // Adds element to end of mutable array
    let push_type = Type::Function(FunctionType {
        params: vec![
            Type::Generic(
                Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
            Type::Generic(
                Ident {
                    name: "U".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Unit, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "push".to_string(),
            ty: Some(push_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'push'");

    // as_int(value) -> int
    // Converts string/float/bool to int
    let as_int_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "as_int".to_string(),
            ty: Some(as_int_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'as_int'");

    // as_float(value) -> float
    // Converts int/string/bool to float
    let as_float_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Float, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "as_float".to_string(),
            ty: Some(as_float_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'as_float'");

    // sqrt(float) -> float
    // Square root
    let sqrt_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::Float, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::Float, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "sqrt".to_string(),
            ty: Some(sqrt_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'sqrt'");

    // pop(array) -> T
    // Removes and returns the last element of a mutable array (panics if empty)
    let pop_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "pop".to_string(),
            ty: Some(pop_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'pop'");

    // enumerate(array) -> Iterator<(int, T)>
    // Returns iterator of (index, element) pairs — valid in for-loop position
    let enumerate_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "Iterator".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "enumerate".to_string(),
            ty: Some(enumerate_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'enumerate'");

    // abs(x) -> int  (overloaded: also float -> float)
    // Absolute value
    let abs_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "abs".to_string(),
            ty: Some(abs_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'abs'");

    // min(int, int) -> int
    let min_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "min".to_string(),
            ty: Some(min_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'min'");

    // max(int, int) -> int
    let max_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "max".to_string(),
            ty: Some(max_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'max'");

    // pow(int, int) -> int
    let pow_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
            Type::Primitive(PrimitiveType::Int, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "pow".to_string(),
            ty: Some(pow_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'pow'");

    // floor(float) -> float
    let floor_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::Float, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::Float, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "floor".to_string(),
            ty: Some(floor_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'floor'");

    // ceil(float) -> float
    let ceil_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::Float, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::Float, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "ceil".to_string(),
            ty: Some(ceil_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'ceil'");

    // round(float) -> float
    let round_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::Float, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::Float, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "round".to_string(),
            ty: Some(round_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'round'");

    // to_string(T) -> string
    let to_string_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "to_string".to_string(),
            ty: Some(to_string_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'to_string'");

    // contains(string, string) -> bool
    let contains_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Bool, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "contains".to_string(),
            ty: Some(contains_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'contains'");

    // split(string, string) -> [string]
    let split_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Array(
            Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "split".to_string(),
            ty: Some(split_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'split'");

    // trim(string) -> string
    let trim_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::String, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "trim".to_string(),
            ty: Some(trim_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'trim'");

    // replace(string, string, string) -> string
    let replace_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "replace".to_string(),
            ty: Some(replace_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'replace'");

    // starts_with(string, string) -> bool
    let starts_with_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Bool, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "starts_with".to_string(),
            ty: Some(starts_with_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'starts_with'");

    // ends_with(string, string) -> bool
    let ends_with_type = Type::Function(FunctionType {
        params: vec![
            Type::Primitive(PrimitiveType::String, Span::dummy()),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Bool, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "ends_with".to_string(),
            ty: Some(ends_with_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'ends_with'");

    // to_upper(string) -> string
    let to_upper_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::String, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "to_upper".to_string(),
            ty: Some(to_upper_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'to_upper'");

    // to_lower(string) -> string
    let to_lower_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::String, Span::dummy())],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "to_lower".to_string(),
            ty: Some(to_lower_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'to_lower'");

    // chars(string) -> [string]
    let chars_type = Type::Function(FunctionType {
        params: vec![Type::Primitive(PrimitiveType::String, Span::dummy())],
        return_type: Box::new(Type::Array(
            Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "chars".to_string(),
            ty: Some(chars_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'chars'");

    // sort(mut [T]) -> unit
    let sort_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Unit, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "sort".to_string(),
            ty: Some(sort_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'sort'");

    // reverse(mut [T]) -> unit
    let reverse_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Unit, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "reverse".to_string(),
            ty: Some(reverse_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'reverse'");

    // map([T], fn(T)->U) -> [U]
    let map_type = Type::Function(FunctionType {
        params: vec![
            Type::Generic(
                Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
            Type::Generic(
                Ident {
                    name: "F".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
        ],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "U".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "map".to_string(),
            ty: Some(map_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'map'");

    // filter([T], fn(T)->bool) -> [T]
    let filter_type = Type::Function(FunctionType {
        params: vec![
            Type::Generic(
                Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
            Type::Generic(
                Ident {
                    name: "F".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
        ],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "filter".to_string(),
            ty: Some(filter_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'filter'");

    // is_empty([T]) -> bool
    let is_empty_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Primitive(PrimitiveType::Bool, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "is_empty".to_string(),
            ty: Some(is_empty_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'is_empty'");

    // concat([T], [T]) -> [T]
    let concat_type = Type::Function(FunctionType {
        params: vec![
            Type::Generic(
                Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
            Type::Generic(
                Ident {
                    name: "T".to_string(),
                    span: Span::dummy(),
                },
                None,
                Span::dummy(),
            ),
        ],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "concat".to_string(),
            ty: Some(concat_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'concat'");

    // join([string], string) -> string
    let join_type = Type::Function(FunctionType {
        params: vec![
            Type::Array(
                Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
                Span::dummy(),
            ),
            Type::Primitive(PrimitiveType::String, Span::dummy()),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::String, Span::dummy())),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "join".to_string(),
            ty: Some(join_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'join'");

    // first([T]) -> T
    let first_type = Type::Function(FunctionType {
        params: vec![Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )],
        return_type: Box::new(Type::Generic(
            Ident {
                name: "T".to_string(),
                span: Span::dummy(),
            },
            None,
            Span::dummy(),
        )),
        span: Span::dummy(),
    });

    symtab
        .insert(Symbol {
            name: "first".to_string(),
            ty: Some(first_type),
            span: Span::dummy(),
            kind: SymbolKind::Function,
            is_builtin: true,
        })
        .expect("Failed to insert builtin 'first'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtins_initialized() {
        let mut symtab = SymbolTable::new();
        initialize_builtins(&mut symtab);

        // Verify print exists
        let print_symbol = symtab.lookup("print");
        assert!(print_symbol.is_some());
        assert_eq!(print_symbol.unwrap().name, "print");
        assert!(matches!(print_symbol.unwrap().kind, SymbolKind::Function));

        // Verify range exists
        let range_symbol = symtab.lookup("range");
        assert!(range_symbol.is_some());
        assert_eq!(range_symbol.unwrap().name, "range");
        assert!(matches!(range_symbol.unwrap().kind, SymbolKind::Function));
    }

    #[test]
    fn test_print_type() {
        let mut symtab = SymbolTable::new();
        initialize_builtins(&mut symtab);

        let print_symbol = symtab.lookup("print").unwrap();
        let print_type = print_symbol.ty.as_ref().expect("print should have a type");

        if let Type::Function(func_type) = print_type {
            assert_eq!(func_type.params.len(), 1);
            assert!(matches!(
                func_type.params[0],
                Type::Primitive(PrimitiveType::String, _)
            ));
            assert!(matches!(
                *func_type.return_type,
                Type::Primitive(PrimitiveType::Unit, _)
            ));
        } else {
            panic!("print should have function type");
        }
    }

    #[test]
    fn test_range_type() {
        let mut symtab = SymbolTable::new();
        initialize_builtins(&mut symtab);

        let range_symbol = symtab.lookup("range").unwrap();
        let range_type = range_symbol.ty.as_ref().expect("range should have a type");

        if let Type::Function(func_type) = range_type {
            assert_eq!(func_type.params.len(), 2);
            assert!(matches!(
                func_type.params[0],
                Type::Primitive(PrimitiveType::Int, _)
            ));
            assert!(matches!(
                func_type.params[1],
                Type::Primitive(PrimitiveType::Int, _)
            ));
            // Return type should be Iterator<int> (Generic type)
            assert!(matches!(*func_type.return_type, Type::Generic(_, _, _)));
        } else {
            panic!("range should have function type");
        }
    }
}
