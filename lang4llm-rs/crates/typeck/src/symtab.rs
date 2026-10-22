//! Symbol table for name resolution
//!
//! Tracks variables, functions, types, and their scopes.

use lang4llm_ast::{Ident, Span, Type};
use std::collections::HashMap;

/// Symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol type annotation (if known)
    pub ty: Option<Type>,
    /// Source location where symbol is defined
    pub span: Span,
    /// Symbol kind
    pub kind: SymbolKind,
    /// True if this symbol was registered as a stdlib builtin.
    /// Builtins can be shadowed by user-defined symbols (DR-TOK-003).
    pub is_builtin: bool,
}

/// Kind of symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// Variable
    Variable,
    /// Function
    Function,
    /// Type (struct, enum, trait)
    Type,
    /// Generic type parameter
    GenericParam,
}

/// Scope in the symbol table
#[derive(Debug, Clone)]
struct Scope {
    /// Symbols in this scope
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    /// Create a new empty scope
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    /// Insert a symbol into this scope
    ///
    /// Returns the previous symbol if one existed with the same name.
    fn insert(&mut self, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(symbol.name.clone(), symbol)
    }

    /// Look up a symbol by name in this scope
    fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

/// Symbol table with scope stack
///
/// Implements lexical scoping with no shadowing allowed.
#[derive(Debug)]
pub struct SymbolTable {
    /// Stack of scopes (innermost scope is last)
    scopes: Vec<Scope>,
}

impl SymbolTable {
    /// Create a new symbol table with global scope
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    /// Push a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Pop the innermost scope from the stack
    ///
    /// # Panics
    ///
    /// Panics if trying to pop the global scope.
    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "Cannot pop global scope");
        self.scopes.pop();
    }

    /// Insert a symbol into the current scope
    ///
    /// # Errors
    ///
    /// Returns `SymbolError::Shadowing` if a non-builtin symbol with the same
    /// name already exists in any visible scope (shadowing is not allowed per
    /// DR-SEM-003). Builtin symbols may be shadowed by user-defined symbols.
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), SymbolError> {
        // Check for shadowing in all visible scopes.
        // Builtins are exempt: user code may introduce variables or functions
        // with the same name as a stdlib builtin.
        if let Some(existing) = self.lookup(&symbol.name) {
            if !existing.is_builtin {
                return Err(SymbolError::Shadowing {
                    name: symbol.name.clone(),
                    original: existing.span,
                    shadowing: symbol.span,
                });
            }
        }

        // Insert into current scope
        let current_scope = self.scopes.last_mut().expect("No scope available");
        current_scope.insert(symbol);
        Ok(())
    }

    /// Look up a symbol by name in all visible scopes
    ///
    /// Searches from innermost to outermost scope.
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
        }
        None
    }

    /// Look up a variable by identifier
    pub fn lookup_variable(&self, ident: &Ident) -> Result<&Symbol, SymbolError> {
        match self.lookup(&ident.name) {
            Some(symbol) if symbol.kind == SymbolKind::Variable => Ok(symbol),
            Some(symbol) => Err(SymbolError::WrongKind {
                name: ident.name.clone(),
                expected: SymbolKind::Variable,
                found: symbol.kind,
                span: ident.span,
            }),
            None => Err(SymbolError::Undefined {
                name: ident.name.clone(),
                span: ident.span,
            }),
        }
    }

    /// Look up a function by identifier
    pub fn lookup_function(&self, ident: &Ident) -> Result<&Symbol, SymbolError> {
        match self.lookup(&ident.name) {
            Some(symbol) if symbol.kind == SymbolKind::Function => Ok(symbol),
            Some(symbol) => Err(SymbolError::WrongKind {
                name: ident.name.clone(),
                expected: SymbolKind::Function,
                found: symbol.kind,
                span: ident.span,
            }),
            None => Err(SymbolError::Undefined {
                name: ident.name.clone(),
                span: ident.span,
            }),
        }
    }

    /// Look up a type by identifier
    pub fn lookup_type(&self, ident: &Ident) -> Result<&Symbol, SymbolError> {
        match self.lookup(&ident.name) {
            Some(symbol) if symbol.kind == SymbolKind::Type => Ok(symbol),
            Some(symbol) => Err(SymbolError::WrongKind {
                name: ident.name.clone(),
                expected: SymbolKind::Type,
                found: symbol.kind,
                span: ident.span,
            }),
            None => Err(SymbolError::Undefined {
                name: ident.name.clone(),
                span: ident.span,
            }),
        }
    }

    /// Check if a symbol exists in any visible scope
    pub fn exists(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Get the current scope depth (0 = global scope)
    pub fn depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Return all names visible in the current scope (for "did you mean?" suggestions).
    ///
    /// Collects every name across all active scopes, deduplicates, and returns
    /// a sorted Vec. Builtins are included so we can suggest them too.
    pub fn all_names_in_scope(&self) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        for scope in &self.scopes {
            for name in scope.symbols.keys() {
                seen.insert(name.clone());
            }
        }
        let mut names: Vec<String> = seen.into_iter().collect();
        names.sort();
        names
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol table error
#[derive(Debug, Clone, thiserror::Error)]
pub enum SymbolError {
    /// Symbol is undefined
    #[error("Undefined symbol '{name}' at {span}")]
    Undefined {
        /// Symbol name
        name: String,
        /// Source location where undefined symbol was used
        span: Span,
    },

    /// Shadowing is not allowed (DR-SEM-003)
    #[error("Shadowing not allowed: '{name}' at {shadowing} shadows declaration at {original}")]
    Shadowing {
        /// Symbol name
        name: String,
        /// Original declaration location
        original: Span,
        /// Shadowing declaration location
        shadowing: Span,
    },

    /// Symbol exists but has wrong kind
    #[error("Expected {expected:?} but '{name}' is {found:?} at {span}")]
    WrongKind {
        /// Symbol name
        name: String,
        /// Expected symbol kind
        expected: SymbolKind,
        /// Found symbol kind
        found: SymbolKind,
        /// Source location where symbol was used
        span: Span,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_span() -> Span {
        Span {
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 5,
        }
    }

    fn make_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol {
            name: name.to_string(),
            ty: None,
            span: make_span(),
            kind,
            is_builtin: false,
        }
    }

    fn make_builtin_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol {
            name: name.to_string(),
            ty: None,
            span: Span::dummy(),
            kind,
            is_builtin: true,
        }
    }

    fn make_ident(name: &str) -> Ident {
        Ident {
            name: name.to_string(),
            span: make_span(),
        }
    }

    #[test]
    fn test_basic_insert_lookup() {
        let mut symtab = SymbolTable::new();
        let x = make_symbol("x", SymbolKind::Variable);

        symtab.insert(x.clone()).unwrap();

        let found = symtab.lookup("x");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "x");
    }

    #[test]
    fn test_scoped_lookup() {
        let mut symtab = SymbolTable::new();

        // Global scope
        let global_x = make_symbol("x", SymbolKind::Variable);
        symtab.insert(global_x).unwrap();

        // Inner scope
        symtab.push_scope();
        let local_y = make_symbol("y", SymbolKind::Variable);
        symtab.insert(local_y).unwrap();

        // Both visible in inner scope
        assert!(symtab.lookup("x").is_some());
        assert!(symtab.lookup("y").is_some());

        // Pop inner scope
        symtab.pop_scope();

        // Only global visible now
        assert!(symtab.lookup("x").is_some());
        assert!(symtab.lookup("y").is_none());
    }

    #[test]
    fn test_no_shadowing() {
        let mut symtab = SymbolTable::new();

        // Insert x in global scope
        let x1 = make_symbol("x", SymbolKind::Variable);
        symtab.insert(x1).unwrap();

        // Try to shadow in inner scope - should fail
        symtab.push_scope();
        let x2 = make_symbol("x", SymbolKind::Variable);
        let result = symtab.insert(x2);

        assert!(result.is_err());
        match result.unwrap_err() {
            SymbolError::Shadowing { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected Shadowing error"),
        }
    }

    #[test]
    fn test_builtin_can_be_shadowed() {
        let mut symtab = SymbolTable::new();

        // Insert builtin `first` in global scope
        let builtin = make_builtin_symbol("first", SymbolKind::Function);
        symtab.insert(builtin).unwrap();

        // User variable `first` should be allowed to shadow the builtin
        symtab.push_scope();
        let user_var = make_symbol("first", SymbolKind::Variable);
        let result = symtab.insert(user_var);
        assert!(result.is_ok(), "Builtins should be shadowable by user code");

        // User variable should now be visible
        let found = symtab.lookup_variable(&make_ident("first"));
        assert!(found.is_ok());
    }

    #[test]
    fn test_lookup_by_kind() {
        let mut symtab = SymbolTable::new();

        let var_x = make_symbol("x", SymbolKind::Variable);
        let func_f = make_symbol("f", SymbolKind::Function);
        let type_t = make_symbol("T", SymbolKind::Type);

        symtab.insert(var_x).unwrap();
        symtab.insert(func_f).unwrap();
        symtab.insert(type_t).unwrap();

        // Correct kind lookups
        assert!(symtab.lookup_variable(&make_ident("x")).is_ok());
        assert!(symtab.lookup_function(&make_ident("f")).is_ok());
        assert!(symtab.lookup_type(&make_ident("T")).is_ok());

        // Wrong kind lookups
        assert!(symtab.lookup_function(&make_ident("x")).is_err());
        assert!(symtab.lookup_type(&make_ident("f")).is_err());
        assert!(symtab.lookup_variable(&make_ident("T")).is_err());
    }

    #[test]
    fn test_undefined_symbol() {
        let symtab = SymbolTable::new();

        let result = symtab.lookup_variable(&make_ident("undefined"));
        assert!(result.is_err());
        match result.unwrap_err() {
            SymbolError::Undefined { name, .. } => {
                assert_eq!(name, "undefined");
            }
            _ => panic!("Expected Undefined error"),
        }
    }

    #[test]
    fn test_depth() {
        let mut symtab = SymbolTable::new();
        assert_eq!(symtab.depth(), 0);

        symtab.push_scope();
        assert_eq!(symtab.depth(), 1);

        symtab.push_scope();
        assert_eq!(symtab.depth(), 2);

        symtab.pop_scope();
        assert_eq!(symtab.depth(), 1);

        symtab.pop_scope();
        assert_eq!(symtab.depth(), 0);
    }
}
