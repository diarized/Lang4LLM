//! Basic type checking for Lang4LLM
//!
//! Validates type correctness for expressions, statements, and items.

use crate::exhaustive::{ExhaustivenessChecker, ExhaustivenessError};
use crate::symtab::{Symbol, SymbolError, SymbolKind, SymbolTable};
use lang4llm_ast::*;
use std::collections::HashMap;

// ─── "Did you mean?" helpers ──────────────────────────────────────────────────

/// Compute the Levenshtein edit distance between two strings.
#[allow(clippy::needless_range_loop)]
fn levenshtein(s: &str, t: &str) -> usize {
    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();
    let m = sv.len();
    let n = tv.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if sv[i - 1] == tv[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}

/// Find names in `candidates` within edit distance 2 of `query`, capped at 5.
fn find_candidates(query: &str, all_names: &[String]) -> Vec<String> {
    let mut matches: Vec<(usize, &str)> = all_names
        .iter()
        .filter(|n| n.as_str() != query)
        .filter_map(|n| {
            let d = levenshtein(query, n);
            if d <= 2 { Some((d, n.as_str())) } else { None }
        })
        .collect();
    matches.sort_by_key(|(d, n)| (*d, *n));
    matches.into_iter().take(5).map(|(_, n)| n.to_string()).collect()
}

/// Type checking context
pub struct TypeChecker {
    /// Symbol table for name resolution
    symtab: SymbolTable,
    /// Current function return type (for validating return statements)
    current_return_type: Option<Type>,
    /// Exhaustiveness checker for pattern matching
    exhaustive_checker: ExhaustivenessChecker,
    /// Generic type substitutions (for monomorphization)
    type_params: HashMap<String, Type>,
    /// Enum definitions (for exhaustiveness checking)
    enum_defs: HashMap<String, EnumDef>,
    /// Struct definitions (for field type checking)
    struct_defs: HashMap<String, StructDef>,
    /// Methods registered from impl blocks: type_name -> [(method_name, FunctionType)]
    methods: HashMap<String, Vec<(String, Type)>>,
    /// Accumulated errors
    errors: Vec<TypeError>,
}

/// Type checking error
#[derive(Debug, Clone, thiserror::Error)]
pub enum TypeError {
    /// Type mismatch
    #[error("Type mismatch at {span}: expected {expected}, found {found}")]
    TypeMismatch {
        /// Source location
        span: Span,
        /// Expected type
        expected: String,
        /// Found type
        found: String,
    },

    /// Undefined symbol
    #[error("Undefined symbol at {span}: {name}")]
    UndefinedSymbol {
        /// Symbol name
        name: String,
        /// Source location
        span: Span,
        /// "Did you mean?" candidates (names within edit distance 2 of `name`)
        candidates: Vec<String>,
    },

    /// Symbol error (shadowing, wrong kind)
    #[error("Symbol error: {0}")]
    Symbol(#[from] SymbolError),

    /// Missing type annotation (DR-SEM-001)
    #[error("Missing type annotation at {span}: variable '{name}' must have explicit type")]
    MissingTypeAnnotation {
        /// Variable name
        name: String,
        /// Source location
        span: Span,
    },

    /// Uninitialized variable (DR-SEM-004)
    #[error("Uninitialized variable at {span}: variable '{name}' must be initialized")]
    Uninitialized {
        /// Variable name
        name: String,
        /// Source location
        span: Span,
    },

    /// Invalid binary operation
    #[error("Invalid binary operation at {span}: cannot apply {op:?} to {left} and {right}")]
    InvalidBinaryOp {
        /// Operator
        op: BinOp,
        /// Left operand type
        left: String,
        /// Right operand type
        right: String,
        /// Source location
        span: Span,
    },

    /// Argument count mismatch
    #[error("Argument count mismatch at {span}: expected {expected}, found {found}")]
    ArgumentCountMismatch {
        /// Expected count
        expected: usize,
        /// Found count
        found: usize,
        /// Call site location
        span: Span,
        /// Function definition location (for secondary label)
        def_span: Span,
        /// Function name (for diagnostic message)
        fn_name: String,
    },

    /// Exhaustiveness error (pattern matching)
    #[error("Exhaustiveness error: {0}")]
    Exhaustiveness(#[from] ExhaustivenessError),

    /// Generic type parameter mismatch
    #[error("Generic type mismatch at {span}: {message}")]
    GenericMismatch {
        /// Error message
        message: String,
        /// Source location
        span: Span,
    },

    /// Non-exhaustive match (enum-aware)
    #[error("Non-exhaustive match at {span}: missing patterns {missing:?}")]
    NonExhaustiveMatch {
        /// Source location
        span: Span,
        /// Missing patterns
        missing: Vec<String>,
    },

    /// Trait bound violation
    #[error("Trait bound violation at {span}: type {ty} does not implement {trait_name}")]
    TraitBoundViolation {
        /// Type that doesn't implement trait
        ty: String,
        /// Trait name
        trait_name: String,
        /// Source location
        span: Span,
    },

    /// Field not found
    #[error("Field not found at {span}: type {ty} has no field '{field}'")]
    FieldNotFound {
        /// Type name
        ty: String,
        /// Field name
        field: String,
        /// Source location
        span: Span,
    },
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        let mut symtab = SymbolTable::new();

        // Initialize builtin functions (print, range, etc.)
        crate::builtins::initialize_builtins(&mut symtab);

        Self {
            symtab,
            current_return_type: None,
            exhaustive_checker: ExhaustivenessChecker::new(),
            type_params: HashMap::new(),
            enum_defs: HashMap::new(),
            struct_defs: HashMap::new(),
            methods: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // First pass: register all top-level items in symbol table
        for item in &program.items {
            if let Err(e) = self.register_item(item) {
                self.errors.push(e.into());
            }
        }

        // Second pass: type check all items
        for item in &program.items {
            self.check_item(item);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Register an item in the symbol table
    fn register_item(&mut self, item: &Item) -> Result<(), SymbolError> {
        match item {
            Item::Function(f) => {
                let symbol = Symbol {
                    name: f.name.name.clone(),
                    ty: Some(self.function_type(f)),
                    span: f.name.span,
                    kind: SymbolKind::Function,
                    is_builtin: false,
                };
                self.symtab.insert(symbol)?;
            }
            Item::Struct(s) => {
                let symbol = Symbol {
                    name: s.name.name.clone(),
                    ty: None, // Type constructors don't have a simple Type representation
                    span: s.name.span,
                    kind: SymbolKind::Type,
                    is_builtin: false,
                };
                self.symtab.insert(symbol)?;
                // Store struct definition for field type checking
                self.struct_defs.insert(s.name.name.clone(), s.clone());
            }
            Item::Enum(e) => {
                let symbol = Symbol {
                    name: e.name.name.clone(),
                    ty: None,
                    span: e.name.span,
                    kind: SymbolKind::Type,
                    is_builtin: false,
                };
                self.symtab.insert(symbol)?;
                // Store enum definition for exhaustiveness checking
                self.enum_defs.insert(e.name.name.clone(), e.clone());
            }
            Item::Trait(t) => {
                let symbol = Symbol {
                    name: t.name.name.clone(),
                    ty: None,
                    span: t.name.span,
                    kind: SymbolKind::Type,
                    is_builtin: false,
                };
                self.symtab.insert(symbol)?;
            }
            Item::Impl(_) => {
                // Impls don't add symbols to the global namespace
            }
        }
        Ok(())
    }

    /// Construct a function type from function definition
    fn function_type(&self, f: &FunctionDef) -> Type {
        let param_types = f.params.iter().map(|p| p.ty.clone()).collect();
        Type::Function(FunctionType {
            params: param_types,
            return_type: Box::new(f.return_type.clone()),
            span: f.span,
        })
    }

    /// Type check an item
    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => self.check_function(f),
            Item::Struct(s) => self.check_struct(s),
            Item::Enum(e) => self.check_enum(e),
            Item::Trait(t) => self.check_trait(t),
            Item::Impl(i) => self.check_impl(i),
        }
    }

    /// Type check a function
    fn check_function(&mut self, f: &FunctionDef) {
        // Push new scope for function body
        self.symtab.push_scope();

        // Set current return type
        let old_return_type = self.current_return_type.take();
        self.current_return_type = Some(f.return_type.clone());

        // Register parameters
        for param in &f.params {
            match &param.name {
                ParamName::Ident(name) => {
                    let symbol = Symbol {
                        name: name.name.clone(),
                        ty: Some(param.ty.clone()),
                        span: name.span,
                        kind: SymbolKind::Variable,
                    is_builtin: false,
                    };
                    if let Err(e) = self.symtab.insert(symbol) {
                        self.errors.push(e.into());
                    }
                }
                ParamName::SelfParam => {
                    // Register `self` as a variable with the parameter's declared type
                    let symbol = Symbol {
                        name: "self".to_string(),
                        ty: Some(param.ty.clone()),
                        span: Span::dummy(),
                        kind: SymbolKind::Variable,
                    is_builtin: false,
                    };
                    if let Err(e) = self.symtab.insert(symbol) {
                        self.errors.push(e.into());
                    }
                }
                ParamName::GenericBound(name, _bounds) => {
                    let symbol = Symbol {
                        name: name.name.clone(),
                        ty: Some(param.ty.clone()),
                        span: name.span,
                        kind: SymbolKind::Variable,
                    is_builtin: false,
                    };
                    if let Err(e) = self.symtab.insert(symbol) {
                        self.errors.push(e.into());
                    }
                }
            }
        }

        // Type check body
        self.check_block(&f.body);

        // Restore return type
        self.current_return_type = old_return_type;

        // Pop scope
        self.symtab.pop_scope();
    }

    /// Type check a struct definition
    fn check_struct(&mut self, s: &StructDef) {
        // Validate that all field types are well-formed
        for field in &s.fields {
            self.validate_type(&field.ty);
        }

        // Check for duplicate field names
        let mut seen_fields = std::collections::HashSet::new();
        for field in &s.fields {
            if !seen_fields.insert(&field.name.name) {
                self.errors.push(TypeError::UndefinedSymbol {
                    name: format!("duplicate field '{}'", field.name.name),
                    span: field.name.span,
                    candidates: vec![],
                });
            }
        }
    }

    /// Type check an enum definition
    fn check_enum(&mut self, e: &EnumDef) {
        // Validate that all variant types are well-formed
        for variant in &e.variants {
            if let Some(data) = &variant.data {
                for ty in data {
                    self.validate_type(ty);
                }
            }
        }

        // Check for duplicate variant names
        let mut seen_variants = std::collections::HashSet::new();
        for variant in &e.variants {
            if !seen_variants.insert(&variant.name.name) {
                self.errors.push(TypeError::UndefinedSymbol {
                    name: format!("duplicate variant '{}'", variant.name.name),
                    span: variant.name.span,
                    candidates: vec![],
                });
            }
        }
    }

    /// Validate that a type is well-formed
    fn validate_type(&mut self, ty: &Type) {
        match ty {
            Type::Primitive(_, _) | Type::Generic(_, _, _) => {
                // Primitives and generics are always valid
            }
            Type::Array(elem, _) => {
                self.validate_type(elem);
            }
            Type::Tuple(types, _) => {
                for ty in types {
                    self.validate_type(ty);
                }
            }
            Type::Struct(name, _) | Type::Enum(name, _) => {
                // Check that the type exists
                if self.symtab.lookup_type(name).is_err() {
                    let all_names = self.symtab.all_names_in_scope();
                    let candidates = find_candidates(&name.name, &all_names);
                    self.errors.push(TypeError::UndefinedSymbol {
                        name: name.name.clone(),
                        span: name.span,
                        candidates,
                    });
                }
            }
            Type::Function(func) => {
                for param in &func.params {
                    self.validate_type(param);
                }
                self.validate_type(&func.return_type);
            }
            Type::Inferred(_) => {
                // Inferred types are valid; actual type comes from context
            }
        }
    }

    /// Type check a trait (placeholder)
    fn check_trait(&mut self, _t: &TraitDef) {
        // Trait definitions don't need type checking beyond name registration
    }

    /// Type check an impl block
    fn check_impl(&mut self, i: &ImplBlock) {
        let type_name = i.type_name.name.name.clone();

        // Register methods in the methods map
        for method in &i.items {
            let method_type = self.function_type(method);
            self.methods
                .entry(type_name.clone())
                .or_default()
                .push((method.name.name.clone(), method_type));
        }

        // Type check each method body
        for method in &i.items {
            self.check_function(method);
        }
    }

    /// Type check a block expression
    fn check_block(&mut self, block: &BlockExpr) -> Option<Type> {
        let mut last_type = None;

        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }

        if let Some(expr) = &block.expr {
            last_type = self.check_expr(expr);
        }

        last_type
    }

    /// Type check a statement
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let(let_stmt) => self.check_let(let_stmt),
            Stmt::Assign(assign) => self.check_assign(assign),
            Stmt::Expr(expr_stmt) => {
                self.check_expr(&expr_stmt.expr);
            }
            Stmt::Return(ret) => self.check_return(ret),
            Stmt::Break(_) => {
                // Break is valid in loop context (context checking is future work)
            }
            Stmt::Continue(_) => {
                // Continue is valid in loop context (context checking is future work)
            }
            Stmt::While(w) => self.check_while(w),
            Stmt::For(f) => self.check_for(f),
            Stmt::Loop(l) => self.check_loop(l),
            Stmt::Empty(_) => {}
        }
    }

    /// Type check a let statement (DR-SEM-001, DR-SEM-004)
    fn check_let(&mut self, let_stmt: &LetStmt) {
        // DR-SEM-001: Explicit type annotation (enforced by grammar)
        let declared_type = &let_stmt.ty;

        // DR-SEM-004: Mandatory initialization
        if let Some(ref init_expr) = let_stmt.init {
            let init_type = self.check_expr(init_expr);
            if let Some(init_type) = init_type {
                if !self.types_compatible(declared_type, &init_type) {
                    // Use the init expression's best span (ident.span is real after US4)
                    let error_span = self.expr_best_span(init_expr, let_stmt.span);
                    self.errors.push(TypeError::TypeMismatch {
                        span: error_span,
                        expected: self.type_to_string(declared_type),
                        found: self.type_to_string(&init_type),
                    });
                }
            }
        } else {
            // No initializer — report uninitialized variable error
            self.errors.push(TypeError::Uninitialized {
                name: let_stmt.name.name.clone(),
                span: let_stmt.span,
            });
        }

        // Register variable in symbol table
        let symbol = Symbol {
            name: let_stmt.name.name.clone(),
            ty: Some(declared_type.clone()),
            span: let_stmt.name.span,
            kind: SymbolKind::Variable,
                    is_builtin: false,
        };
        if let Err(e) = self.symtab.insert(symbol) {
            self.errors.push(e.into());
        }
    }

    /// Type check an assignment statement
    fn check_assign(&mut self, assign: &AssignStmt) {
        let lhs_type = self.check_assign_target(&assign.lhs);
        let rhs_type = self.check_expr(&assign.rhs);

        if let (Some(lhs_type), Some(rhs_type)) = (lhs_type, rhs_type) {
            if !self.types_compatible(&lhs_type, &rhs_type) {
                self.errors.push(TypeError::TypeMismatch {
                    span: assign.span,
                    expected: self.type_to_string(&lhs_type),
                    found: self.type_to_string(&rhs_type),
                });
            }
        }
    }

    /// Type check an assignment target
    fn check_assign_target(&mut self, target: &AssignTarget) -> Option<Type> {
        match target {
            AssignTarget::Var(ident) => {
                match self.symtab.lookup_variable(ident) {
                    Ok(symbol) => symbol.ty.clone(),
                    Err(e) => {
                        self.errors.push(e.into());
                        None
                    }
                }
            }
            AssignTarget::ArrayAccess(array, index, _) => {
                // Type check array and index
                let array_type = self.check_expr(array);
                self.check_expr(index);

                // Extract element type from array type
                if let Some(Type::Array(elem_type, _)) = array_type {
                    Some(*elem_type)
                } else {
                    None
                }
            }
            AssignTarget::FieldAccess(object, _field, _) => {
                // Type check struct expression
                self.check_expr(object);
                // Field type lookup would require struct type information (future work)
                None
            }
            AssignTarget::Tuple(_patterns, _) => {
                // Tuple destructuring (future work)
                None
            }
        }
    }

    /// Type check a return statement
    fn check_return(&mut self, ret: &ReturnStmt) {
        let return_type = if let Some(expr) = &ret.value {
            self.check_expr(expr)
        } else {
            Some(Type::Primitive(PrimitiveType::Unit, ret.span))
        };

        if let Some(expected_type) = &self.current_return_type {
            if let Some(return_type) = return_type {
                if !self.types_compatible(expected_type, &return_type) {
                    self.errors.push(TypeError::TypeMismatch {
                        span: ret.span,
                        expected: self.type_to_string(expected_type),
                        found: self.type_to_string(&return_type),
                    });
                }
            }
        }
    }

    /// Type check a while statement
    fn check_while(&mut self, w: &WhileStmt) {
        let cond_type = self.check_expr(&w.condition);
        if let Some(cond_type) = cond_type {
            let bool_type = Type::Primitive(PrimitiveType::Bool, w.span);
            if !self.types_compatible(&bool_type, &cond_type) {
                self.errors.push(TypeError::TypeMismatch {
                    span: w.span,
                    expected: "Bool".to_string(),
                    found: self.type_to_string(&cond_type),
                });
            }
        }
        self.check_block(&w.body);
    }

    /// Type check a for statement
    fn check_for(&mut self, f: &ForStmt) {
        self.symtab.push_scope();

        // Check iterator expression
        self.check_expr(&f.iter);

        // Register loop variable
        let symbol = Symbol {
            name: f.var.name.clone(),
            ty: None, // Type would be inferred from iterable (future work)
            span: f.var.span,
            kind: SymbolKind::Variable,
                    is_builtin: false,
        };
        if let Err(e) = self.symtab.insert(symbol) {
            self.errors.push(e.into());
        }

        self.check_block(&f.body);
        self.symtab.pop_scope();
    }

    /// Type check a loop statement
    fn check_loop(&mut self, l: &LoopStmt) {
        self.check_block(&l.body);
    }

    /// Type check an expression
    fn check_expr(&mut self, expr: &Expr) -> Option<Type> {
        match expr {
            Expr::Literal(lit, span) => Some(self.check_literal(lit, *span)),
            Expr::Variable(ident, _) => self.check_variable(ident),
            Expr::Binary(bin) => self.check_binary(bin),
            Expr::Unary(un) => self.check_unary(un),
            Expr::Call(call) => self.check_call(call),
            Expr::Array(arr) => self.check_array_literal(arr),
            Expr::Tuple(tup) => self.check_tuple_literal(tup),
            Expr::Struct(s) => self.check_struct_literal(s),
            Expr::ArrayAccess(access) => self.check_array_access(access),
            Expr::FieldAccess(access) => self.check_field_access(access),
            Expr::Paren(expr, _) => self.check_expr(expr),
            Expr::If(if_expr) => self.check_if(if_expr),
            Expr::Match(match_expr) => self.check_match(match_expr),
            Expr::Block(block) => self.check_block(block),
            Expr::Range(range) => self.check_range(range),
            Expr::Closure(_) => {
                // Closure type checking (future work)
                None
            }
            Expr::EnumVariant(_) => {
                // Enum variant literal (future work)
                None
            }
        }
    }

    /// Return the best available span for an expression.
    /// For Variable expressions, `ident.span` is real (after US4 span fix).
    /// For all other expressions, falls back to `fallback`.
    fn expr_best_span(&self, expr: &Expr, fallback: Span) -> Span {
        match expr {
            Expr::Variable(ident, _) => ident.span,
            Expr::Literal(_, span) => *span,
            Expr::Paren(_, span) => *span,
            _ => fallback,
        }
    }

    /// Type check a literal
    fn check_literal(&self, lit: &Literal, span: Span) -> Type {
        match lit {
            Literal::Int(_) => Type::Primitive(PrimitiveType::Int, span),
            Literal::Float(_) => Type::Primitive(PrimitiveType::Float, span),
            Literal::Bool(_) => Type::Primitive(PrimitiveType::Bool, span),
            Literal::String(_) => Type::Primitive(PrimitiveType::String, span),
            Literal::None => Type::Primitive(PrimitiveType::Unit, span),
        }
    }

    /// Type check a variable reference
    fn check_variable(&mut self, ident: &Ident) -> Option<Type> {
        match self.symtab.lookup_variable(ident) {
            Ok(symbol) => symbol.ty.clone(),
            Err(e) => {
                self.errors.push(e.into());
                None
            }
        }
    }

    /// Type check a binary expression
    fn check_binary(&mut self, bin: &BinaryExpr) -> Option<Type> {
        let left_type = self.check_expr(&bin.lhs)?;
        let right_type = self.check_expr(&bin.rhs)?;

        // Check operator validity for types
        match bin.op {
            // Special case for Add: Support string concatenation
            BinOp::Add => {
                // String + anything → String (string concatenation with implicit conversion)
                if self.is_string(&left_type) || self.is_string(&right_type) {
                    Some(Type::Primitive(PrimitiveType::String, bin.span))
                }
                // Numeric + Numeric → Numeric (arithmetic)
                else if self.is_numeric(&left_type) && self.types_compatible(&left_type, &right_type) {
                    Some(left_type)
                } else {
                    self.errors.push(TypeError::InvalidBinaryOp {
                        op: bin.op,
                        left: self.type_to_string(&left_type),
                        right: self.type_to_string(&right_type),
                        span: bin.span,
                    });
                    None
                }
            }
            // Arithmetic: Int/Float × Int/Float → Int/Float
            BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                if self.is_numeric(&left_type) && self.types_compatible(&left_type, &right_type) {
                    Some(left_type)
                } else {
                    self.errors.push(TypeError::InvalidBinaryOp {
                        op: bin.op,
                        left: self.type_to_string(&left_type),
                        right: self.type_to_string(&right_type),
                        span: bin.span,
                    });
                    None
                }
            }
            // Comparison: T × T → Bool (where T is comparable)
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                if self.types_compatible(&left_type, &right_type) {
                    Some(Type::Primitive(PrimitiveType::Bool, bin.span))
                } else {
                    self.errors.push(TypeError::InvalidBinaryOp {
                        op: bin.op,
                        left: self.type_to_string(&left_type),
                        right: self.type_to_string(&right_type),
                        span: bin.span,
                    });
                    None
                }
            }
            // Logical: Bool × Bool → Bool
            BinOp::And | BinOp::Or => {
                let bool_type = Type::Primitive(PrimitiveType::Bool, bin.span);
                if self.types_compatible(&left_type, &bool_type)
                    && self.types_compatible(&right_type, &bool_type)
                {
                    Some(bool_type)
                } else {
                    self.errors.push(TypeError::InvalidBinaryOp {
                        op: bin.op,
                        left: self.type_to_string(&left_type),
                        right: self.type_to_string(&right_type),
                        span: bin.span,
                    });
                    None
                }
            }
        }
    }

    /// Type check a unary expression
    fn check_unary(&mut self, un: &UnaryExpr) -> Option<Type> {
        let operand_type = self.check_expr(&un.operand)?;

        match un.op {
            UnOp::Neg => {
                // Numeric negation: -Int → Int, -Float → Float
                if self.is_numeric(&operand_type) {
                    Some(operand_type)
                } else {
                    None
                }
            }
            UnOp::Not => {
                // Logical not: !Bool → Bool
                let bool_type = Type::Primitive(PrimitiveType::Bool, un.span);
                if self.types_compatible(&operand_type, &bool_type) {
                    Some(bool_type)
                } else {
                    None
                }
            }
        }
    }

    /// Type check a function call
    fn check_call(&mut self, call: &CallExpr) -> Option<Type> {
        // Extract function identifier from expression
        let func_ident = match call.func.as_ref() {
            Expr::Variable(ident, _) => ident,
            Expr::FieldAccess(fa) => {
                // Method call: obj.method(args)
                // Type check the object but don't enforce method resolution yet
                self.check_expr(&Expr::FieldAccess(fa.clone()));
                return None;
            }
            _ => {
                // Other callable expressions not yet supported
                return None;
            }
        };

        // Look up function — try as function first, then as variable with function type
        let (func_type_owned, func_def_span) = match self.symtab.lookup_function(func_ident) {
            Ok(symbol) => (symbol.ty.clone(), symbol.span),
            Err(_) => {
                // Try as variable (callable variable like closures or function-typed values)
                match self.symtab.lookup_variable(func_ident) {
                    Ok(symbol) => (symbol.ty.clone(), symbol.span),
                    Err(e) => {
                        self.errors.push(e.into());
                        return None;
                    }
                }
            }
        };

        // Extract function type
        let func_type = func_type_owned.as_ref()?;
        if let Type::Function(fn_type) = func_type {
            // Check argument count
            if call.args.len() != fn_type.params.len() {
                self.errors.push(TypeError::ArgumentCountMismatch {
                    expected: fn_type.params.len(),
                    found: call.args.len(),
                    span: call.span,
                    def_span: func_def_span,
                    fn_name: func_ident.name.clone(),
                });
                return None;
            }

            // Special case: contains([T], T) -> bool (overloaded builtin)
            // The registered type is (string, string) -> bool, but we also accept
            // (array, element) where element type matches the array element type.
            if func_ident.name == "contains" && call.args.len() == 2 {
                let arg0_type = self.check_expr(&call.args[0]);
                let arg1_type = self.check_expr(&call.args[1]);
                if let Some(Type::Array(elem_ty, _)) = &arg0_type {
                    // Array variant: check that arg1 is compatible with element type
                    if let Some(ref a1) = arg1_type {
                        if !self.types_compatible(elem_ty, a1) {
                            self.errors.push(TypeError::TypeMismatch {
                                span: call.span,
                                expected: self.type_to_string(elem_ty),
                                found: self.type_to_string(a1),
                            });
                        }
                    }
                    return Some(Type::Primitive(PrimitiveType::Bool, call.span));
                }
                // Fall through to normal checking for string variant
            }

            // Clone what we need before mutable borrows
            let param_types = fn_type.params.clone();
            let return_type = *fn_type.return_type.clone();

            // Check argument types
            for (arg, param_type) in call.args.iter().zip(&param_types) {
                if let Some(arg_type) = self.check_expr(arg) {
                    if !self.types_compatible(param_type, &arg_type) {
                        self.errors.push(TypeError::TypeMismatch {
                            span: call.span,
                            expected: self.type_to_string(param_type),
                            found: self.type_to_string(&arg_type),
                        });
                    }
                }
            }

            // Return function return type
            Some(return_type)
        } else {
            None
        }
    }

    /// Type check array literal (placeholder)
    fn check_array_literal(&mut self, arr: &ArrayLiteral) -> Option<Type> {
        // Check all elements have same type
        if arr.elements.is_empty() {
            return None;
        }

        let first_type = self.check_expr(&arr.elements[0])?;
        for elem in &arr.elements[1..] {
            if let Some(elem_type) = self.check_expr(elem) {
                if !self.types_compatible(&first_type, &elem_type) {
                    self.errors.push(TypeError::TypeMismatch {
                        span: arr.span,
                        expected: self.type_to_string(&first_type),
                        found: self.type_to_string(&elem_type),
                    });
                }
            }
        }

        Some(Type::Array(Box::new(first_type), arr.span))
    }

    /// Type check tuple literal (placeholder)
    fn check_tuple_literal(&mut self, tup: &TupleLiteral) -> Option<Type> {
        let elem_types: Vec<Type> = tup
            .elements
            .iter()
            .filter_map(|e| self.check_expr(e))
            .collect();

        Some(Type::Tuple(elem_types, tup.span))
    }

    /// Type check struct literal
    fn check_struct_literal(&mut self, s: &StructLiteral) -> Option<Type> {
        // Look up struct type
        self.symtab.lookup_type(&s.name).ok()?;

        // Clone struct definition to avoid borrow checker issues
        let struct_def_opt = self.struct_defs.get(&s.name.name).cloned();

        if let Some(struct_def) = struct_def_opt {
            // Build a map of provided fields
            let mut provided_fields = std::collections::HashMap::new();
            for field in &s.fields {
                provided_fields.insert(&field.name.name, field);
            }

            // Check all required fields are present and types match
            for field_def in &struct_def.fields {
                if let Some(field_init) = provided_fields.get(&field_def.name.name) {
                    // Type check the initializer
                    if let Some(init_type) = self.check_expr(&field_init.value) {
                        if !self.types_compatible(&field_def.ty, &init_type) {
                            self.errors.push(TypeError::TypeMismatch {
                                span: field_init.span,
                                expected: self.type_to_string(&field_def.ty),
                                found: self.type_to_string(&init_type),
                            });
                        }
                    }
                } else {
                    // Missing required field
                    self.errors.push(TypeError::TypeMismatch {
                        span: s.span,
                        expected: format!("struct {} with field '{}'", s.name.name, field_def.name.name),
                        found: "incomplete struct literal".to_string(),
                    });
                }
            }

            // Check for extra fields not in the struct definition
            for field_init in &s.fields {
                let field_exists = struct_def.fields.iter().any(|f| f.name.name == field_init.name.name);
                if !field_exists {
                    self.errors.push(TypeError::FieldNotFound {
                        ty: s.name.name.clone(),
                        field: field_init.name.name.clone(),
                        span: field_init.span,
                    });
                }
            }
        }

        // Return as Generic type (matches how parser produces types from type annotations)
        Some(Type::Generic(s.name.clone(), None, s.span))
    }

    /// Type check array access (placeholder)
    fn check_array_access(&mut self, access: &ArrayAccess) -> Option<Type> {
        let array_type = self.check_expr(&access.array)?;
        let index_type = self.check_expr(&access.index)?;

        // Check index is Int
        let int_type = Type::Primitive(PrimitiveType::Int, access.span);
        if !self.types_compatible(&int_type, &index_type) {
            self.errors.push(TypeError::TypeMismatch {
                span: access.span,
                expected: "Int".to_string(),
                found: self.type_to_string(&index_type),
            });
        }

        // Extract element type from array
        if let Type::Array(elem_type, _) = array_type {
            Some(*elem_type)
        } else {
            None
        }
    }

    /// Type check field access
    fn check_field_access(&mut self, access: &FieldAccess) -> Option<Type> {
        let object_type = self.check_expr(&access.object)?;

        // Extract the type name to look up struct definition
        let type_name = match &object_type {
            Type::Struct(name, _) => Some(name.name.clone()),
            // Parser produces Type::Generic for user-defined types (e.g., Task, Point)
            Type::Generic(name, _, _) => Some(name.name.clone()),
            _ => None,
        };

        if let Some(type_name) = type_name {
            // First check struct fields
            if let Some(struct_def) = self.struct_defs.get(&type_name).cloned() {
                for field in &struct_def.fields {
                    if field.name.name == access.field.name {
                        return Some(field.ty.clone());
                    }
                }
            }

            // Then check methods from impl blocks
            if let Some(type_methods) = self.methods.get(&type_name).cloned() {
                for (method_name, method_type) in &type_methods {
                    if *method_name == access.field.name {
                        // Return the function type — it will be called via PostfixExpr
                        return Some(method_type.clone());
                    }
                }
            }

            // Neither field nor method found
            if self.struct_defs.contains_key(&type_name) || self.methods.contains_key(&type_name) {
                self.errors.push(TypeError::FieldNotFound {
                    ty: type_name,
                    field: access.field.name.clone(),
                    span: access.span,
                });
                None
            } else {
                // Not a known type — allow without error
                None
            }
        } else {
            self.errors.push(TypeError::TypeMismatch {
                span: access.span,
                expected: "struct type".to_string(),
                found: self.type_to_string(&object_type),
            });
            None
        }
    }

    /// Type check if expression
    fn check_if(&mut self, if_expr: &IfExpr) -> Option<Type> {
        let cond_type = self.check_expr(&if_expr.condition)?;
        let bool_type = Type::Primitive(PrimitiveType::Bool, if_expr.span);
        if !self.types_compatible(&bool_type, &cond_type) {
            self.errors.push(TypeError::TypeMismatch {
                span: if_expr.span,
                expected: "Bool".to_string(),
                found: self.type_to_string(&cond_type),
            });
        }

        let then_type = self.check_block(&if_expr.then_branch);

        // If without else: type is unit (statement context)
        let else_type = if let Some(else_branch) = &if_expr.else_branch {
            self.check_block(else_branch)
        } else {
            return Some(Type::Primitive(PrimitiveType::Unit, if_expr.span));
        };

        // Both branches should have compatible types
        if let (Some(then_type), Some(else_type)) = (then_type, else_type) {
            if self.types_compatible(&then_type, &else_type) {
                return Some(then_type);
            }
        }

        // If incompatible types, return Unit
        Some(Type::Primitive(
            PrimitiveType::Unit,
            if_expr.span,
        ))
    }

    /// Type check match expression
    fn check_match(&mut self, match_expr: &MatchExpr) -> Option<Type> {
        // Type check the matched expression
        let matched_type = self.check_expr(&match_expr.expr)?;

        // Check exhaustiveness (DR-ERR-004)
        // First, do enum-specific exhaustiveness check using enum_defs
        let enum_name = match &matched_type {
            Type::Generic(name, _, _) if self.enum_defs.contains_key(&name.name) => Some(name.name.clone()),
            Type::Enum(name, _) => Some(name.name.clone()),
            _ => None,
        };
        if let Some(ref ename) = enum_name {
            if let Some(enum_def) = self.enum_defs.get(ename) {
                let all_variants: std::collections::HashSet<String> = enum_def.variants.iter()
                    .map(|v| v.name.name.clone()).collect();
                let mut covered: std::collections::HashSet<String> = std::collections::HashSet::new();
                let mut has_wildcard = false;
                for arm in &match_expr.arms {
                    match &arm.pattern {
                        Pattern::Enum(ep) => { covered.insert(ep.variant_name.name.clone()); }
                        Pattern::Wildcard(_) => { has_wildcard = true; }
                        _ => {}
                    }
                }
                if !has_wildcard {
                    let missing: Vec<String> = all_variants.difference(&covered)
                        .map(|v| format!("{}::{}", ename, v)).collect();
                    if !missing.is_empty() {
                        self.errors.push(TypeError::NonExhaustiveMatch {
                            span: match_expr.span,
                            missing,
                        });
                    }
                }
            }
        } else if let Err(errors) = self.exhaustive_checker.check_match_exhaustive(match_expr, &matched_type) {
            for error in errors {
                self.errors.push(error.into());
            }
        }

        // Type check all arms
        let mut arm_types = Vec::new();
        for arm in &match_expr.arms {
            // Push new scope for pattern bindings
            self.symtab.push_scope();

            // Type check pattern and register bindings
            self.check_pattern(&arm.pattern, &matched_type);

            // Type check arm body
            if let Some(arm_type) = self.check_expr(&arm.body) {
                arm_types.push(arm_type);
            }

            // Pop scope
            self.symtab.pop_scope();
        }

        // All arms should have compatible types
        if let Some(first_type) = arm_types.first() {
            for (i, arm_type) in arm_types.iter().enumerate().skip(1) {
                if !self.types_compatible(first_type, arm_type) {
                    self.errors.push(TypeError::TypeMismatch {
                        span: match_expr.arms[i].span,
                        expected: self.type_to_string(first_type),
                        found: self.type_to_string(arm_type),
                    });
                }
            }
            Some(first_type.clone())
        } else {
            Some(Type::Primitive(PrimitiveType::Unit, match_expr.span))
        }
    }

    /// Type check a pattern and register bindings
    fn check_pattern(&mut self, pattern: &Pattern, expected_type: &Type) {
        match pattern {
            Pattern::Literal(lit, span) => {
                let lit_type = self.check_literal(lit, *span);
                if !self.types_compatible(expected_type, &lit_type) {
                    self.errors.push(TypeError::TypeMismatch {
                        span: *span,
                        expected: self.type_to_string(expected_type),
                        found: self.type_to_string(&lit_type),
                    });
                }
            }
            Pattern::Ident(ident, _) => {
                // Bind pattern variable with expected type
                let symbol = Symbol {
                    name: ident.name.clone(),
                    ty: Some(expected_type.clone()),
                    span: ident.span,
                    kind: SymbolKind::Variable,
                    is_builtin: false,
                };
                if let Err(e) = self.symtab.insert(symbol) {
                    self.errors.push(e.into());
                }
            }
            Pattern::Wildcard(_) => {
                // Wildcard matches anything, no binding
            }
            Pattern::Tuple(patterns, span) => {
                // Check tuple pattern against tuple type
                if let Type::Tuple(types, _) = expected_type {
                    if patterns.len() != types.len() {
                        self.errors.push(TypeError::TypeMismatch {
                            span: *span,
                            expected: self.type_to_string(expected_type),
                            found: format!("tuple with {} elements", patterns.len()),
                        });
                    } else {
                        for (pat, ty) in patterns.iter().zip(types) {
                            self.check_pattern(pat, ty);
                        }
                    }
                } else {
                    self.errors.push(TypeError::TypeMismatch {
                        span: *span,
                        expected: self.type_to_string(expected_type),
                        found: "tuple pattern".to_string(),
                    });
                }
            }
            Pattern::Struct(struct_pat) => {
                // Check struct pattern against struct type
                if let Type::Struct(struct_name, _) = expected_type {
                    if struct_name.name != struct_pat.name.name {
                        self.errors.push(TypeError::TypeMismatch {
                            span: struct_pat.name.span,
                            expected: struct_name.name.clone(),
                            found: struct_pat.name.name.clone(),
                        });
                    }
                    // Check field patterns (requires struct field types - future enhancement)
                } else {
                    self.errors.push(TypeError::TypeMismatch {
                        span: struct_pat.name.span,
                        expected: self.type_to_string(expected_type),
                        found: "struct pattern".to_string(),
                    });
                }
            }
            Pattern::Enum(enum_pat) => {
                // Extract enum name from expected type
                let enum_name = match expected_type {
                    Type::Enum(name, _) => Some(name.name.clone()),
                    // Parser produces Type::Generic for user-defined types
                    Type::Generic(name, _, _) => {
                        if self.enum_defs.contains_key(&name.name) {
                            Some(name.name.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(enum_type_name) = enum_name {
                    // Validate that the variant belongs to this enum
                    if let Some(pattern_enum_name) = &enum_pat.enum_name {
                        if pattern_enum_name.name != enum_type_name {
                            self.errors.push(TypeError::TypeMismatch {
                                span: enum_pat.span,
                                expected: enum_type_name.clone(),
                                found: pattern_enum_name.name.clone(),
                            });
                        }
                    }
                    // Register pattern bindings from variant fields
                    if let Some(patterns) = &enum_pat.patterns {
                        // Look up variant field types from enum definition
                        let variant_field_types: Vec<Type> = self.enum_defs
                            .get(&enum_type_name)
                            .and_then(|enum_def| {
                                enum_def.variants.iter().find(|v| v.name.name == enum_pat.variant_name.name)
                            })
                            .and_then(|variant| variant.data.clone())
                            .unwrap_or_default();

                        for (idx, pat) in patterns.iter().enumerate() {
                            if let Pattern::Ident(ident, _) = pat {
                                let field_type = variant_field_types.get(idx).cloned().unwrap_or_else(|| {
                                    Type::Generic(
                                        Ident { name: "T".to_string(), span: Span::dummy() },
                                        None,
                                        Span::dummy(),
                                    )
                                });
                                let symbol = Symbol {
                                    name: ident.name.clone(),
                                    ty: Some(field_type),
                                    span: ident.span,
                                    kind: SymbolKind::Variable,
                    is_builtin: false,
                                };
                                if let Err(e) = self.symtab.insert(symbol) {
                                    self.errors.push(e.into());
                                }
                            }
                        }
                    }
                } else {
                    // Not a known enum type — could be a type variable, allow it
                    if let Some(patterns) = &enum_pat.patterns {
                        for pat in patterns {
                            if let Pattern::Ident(ident, _) = pat {
                                let symbol = Symbol {
                                    name: ident.name.clone(),
                                    ty: Some(Type::Generic(
                                        Ident { name: "T".to_string(), span: Span::dummy() },
                                        None,
                                        Span::dummy(),
                                    )),
                                    span: ident.span,
                                    kind: SymbolKind::Variable,
                    is_builtin: false,
                                };
                                if let Err(e) = self.symtab.insert(symbol) {
                                    self.errors.push(e.into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Type check range expression (placeholder)
    fn check_range(&mut self, range: &RangeExpr) -> Option<Type> {
        if let Some(start) = &range.start {
            self.check_expr(start);
        }
        if let Some(end) = &range.end {
            self.check_expr(end);
        }
        // Range type would be a generic iterator type (future work)
        None
    }

    /// Check if two types are compatible
    fn types_compatible(&self, expected: &Type, found: &Type) -> bool {
        match (expected, found) {
            (Type::Primitive(p1, _), Type::Primitive(p2, _)) => p1 == p2,
            (Type::Array(e1, _), Type::Array(e2, _)) => self.types_compatible(e1, e2),
            (Type::Tuple(t1, _), Type::Tuple(t2, _)) => {
                t1.len() == t2.len()
                    && t1
                        .iter()
                        .zip(t2)
                        .all(|(ty1, ty2)| self.types_compatible(ty1, ty2))
            }
            (Type::Struct(n1, _), Type::Struct(n2, _)) => n1.name == n2.name,
            (Type::Enum(n1, _), Type::Enum(n2, _)) => n1.name == n2.name,
            // Generic type parameters act as wildcards:
            // - A single-letter generic name (T, U, E) is a type variable that matches anything
            // - A named generic (Option<T>, Result<T, E>) matches same-name generics
            (Type::Generic(n1, args1, _), Type::Generic(n2, args2, _)) => {
                // Type variables (single uppercase letter, no args) match anything
                if self.is_type_variable(n1) && args1.is_none() {
                    return true;
                }
                if self.is_type_variable(n2) && args2.is_none() {
                    return true;
                }
                // Named generics: compare names (args are erased for now)
                n1.name == n2.name
            }
            // Type variable on either side matches any concrete type
            (Type::Generic(n, None, _), _) if self.is_type_variable(n) => true,
            (_, Type::Generic(n, None, _)) if self.is_type_variable(n) => true,
            (Type::Function(f1), Type::Function(f2)) => {
                self.types_compatible(&f1.return_type, &f2.return_type)
                    && f1.params.len() == f2.params.len()
                    && f1
                        .params
                        .iter()
                        .zip(&f2.params)
                        .all(|(p1, p2)| self.types_compatible(p1, p2))
            }
            _ => false,
        }
    }

    /// Check if an identifier represents a type variable (single uppercase letter or known type param)
    fn is_type_variable(&self, ident: &Ident) -> bool {
        let name = &ident.name;
        // Type parameters registered in scope
        if self.type_params.contains_key(name) {
            return true;
        }
        // Convention: single uppercase letter is a type variable
        name.len() == 1 && name.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
    }

    /// Check if a type is numeric (Int or Float)
    fn is_numeric(&self, ty: &Type) -> bool {
        matches!(
            ty,
            Type::Primitive(PrimitiveType::Int, _) | Type::Primitive(PrimitiveType::Float, _)
        )
    }

    /// Check if a type is string
    fn is_string(&self, ty: &Type) -> bool {
        matches!(ty, Type::Primitive(PrimitiveType::String, _))
    }

    /// Convert type to string for error messages
    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Primitive(p, _) => format!("{:?}", p),
            Type::Array(elem, _) => format!("[{}]", self.type_to_string(elem)),
            Type::Tuple(types, _) => {
                let types_str = types
                    .iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", types_str)
            }
            Type::Struct(name, _) => name.name.clone(),
            Type::Enum(name, _) => name.name.clone(),
            Type::Generic(name, _, _) => name.name.clone(),
            Type::Function(f) => {
                let params = f
                    .params
                    .iter()
                    .map(|p| self.type_to_string(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                let ret = self.type_to_string(&f.return_type);
                format!("fn({}) -> {}", params, ret)
            }
            Type::Inferred(_) => "_".to_string(),
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang4llm_ast::*;

    fn span() -> Span {
        Span::dummy()
    }

    fn ident(name: &str) -> Ident {
        Ident {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int_type() -> Type {
        Type::Primitive(PrimitiveType::Int, span())
    }

    fn bool_type() -> Type {
        Type::Primitive(PrimitiveType::Bool, span())
    }

    fn string_type() -> Type {
        Type::Primitive(PrimitiveType::String, span())
    }

    // =============================================================================
    // Struct Literal Tests
    // =============================================================================

    #[test]
    fn test_struct_literal_complete_fields() {
        let mut checker = TypeChecker::new();

        // Define struct Point { x: Int, y: Int }
        let struct_def = StructDef {
            name: ident("Point"),
            generics: None,
            fields: vec![
                FieldDef {
                    name: ident("x"),
                    ty: int_type(),
                    span: span(),
                },
                FieldDef {
                    name: ident("y"),
                    ty: int_type(),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct(&struct_def);

        // Register struct in struct_defs and symbol table
        checker.struct_defs.insert(struct_def.name.name.clone(), struct_def.clone());
        checker.symtab.insert(Symbol {
            name: struct_def.name.name.clone(),
            ty: None,
            span: struct_def.span,
            kind: SymbolKind::Type,
                    is_builtin: false,
        }).unwrap();

        // Create complete literal: Point { x: 10, y: 20 }
        let literal = StructLiteral {
            name: ident("Point"),
            fields: vec![
                FieldInit {
                    name: ident("x"),
                    value: Box::new(Expr::Literal(Literal::Int(10), span())),
                    span: span(),
                },
                FieldInit {
                    name: ident("y"),
                    value: Box::new(Expr::Literal(Literal::Int(20), span())),
                    span: span(),
                },
            ],
            span: span(),
        };

        let result = checker.check_struct_literal(&literal);
        assert!(result.is_some());
    }

    #[test]
    fn test_struct_literal_missing_field() {
        let mut checker = TypeChecker::new();

        let struct_def = StructDef {
            name: ident("Point"),
            generics: None,
            fields: vec![
                FieldDef {
                    name: ident("x"),
                    ty: int_type(),
                    span: span(),
                },
                FieldDef {
                    name: ident("y"),
                    ty: int_type(),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct(&struct_def);

        // Register struct in struct_defs and symbol table
        checker.struct_defs.insert(struct_def.name.name.clone(), struct_def.clone());
        checker.symtab.insert(Symbol {
            name: struct_def.name.name.clone(),
            ty: None,
            span: struct_def.span,
            kind: SymbolKind::Type,
                    is_builtin: false,
        }).unwrap();

        // Incomplete literal: Point { x: 10 }
        let literal = StructLiteral {
            name: ident("Point"),
            fields: vec![FieldInit {
                name: ident("x"),
                value: Box::new(Expr::Literal(Literal::Int(10), span())),
                span: span(),
            }],
            span: span(),
        };

        checker.check_struct_literal(&literal);
        assert!(!checker.errors.is_empty());
    }

    #[test]
    fn test_struct_literal_wrong_field_type() {
        let mut checker = TypeChecker::new();

        let struct_def = StructDef {
            name: ident("Point"),
            generics: None,
            fields: vec![
                FieldDef {
                    name: ident("x"),
                    ty: int_type(),
                    span: span(),
                },
                FieldDef {
                    name: ident("y"),
                    ty: int_type(),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct(&struct_def);

        // Register struct in struct_defs and symbol table
        checker.struct_defs.insert(struct_def.name.name.clone(), struct_def.clone());
        checker.symtab.insert(Symbol {
            name: struct_def.name.name.clone(),
            ty: None,
            span: struct_def.span,
            kind: SymbolKind::Type,
                    is_builtin: false,
        }).unwrap();

        // Wrong type: Point { x: 10, y: "hello" }
        let literal = StructLiteral {
            name: ident("Point"),
            fields: vec![
                FieldInit {
                    name: ident("x"),
                    value: Box::new(Expr::Literal(Literal::Int(10), span())),
                    span: span(),
                },
                FieldInit {
                    name: ident("y"),
                    value: Box::new(Expr::Literal(Literal::String("hello".to_string()), span())),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct_literal(&literal);
        assert!(!checker.errors.is_empty());
    }

    // =============================================================================
    // Field Access Tests
    // =============================================================================

    #[test]
    fn test_field_access_valid() {
        let mut checker = TypeChecker::new();

        let struct_def = StructDef {
            name: ident("Point"),
            generics: None,
            fields: vec![
                FieldDef {
                    name: ident("x"),
                    ty: int_type(),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct(&struct_def);

        // Register struct in struct_defs and symbol table
        checker.struct_defs.insert(struct_def.name.name.clone(), struct_def.clone());
        checker.symtab.insert(Symbol {
            name: struct_def.name.name.clone(),
            ty: None,
            span: struct_def.span,
            kind: SymbolKind::Type,
                    is_builtin: false,
        }).unwrap();

        checker.symtab.insert(Symbol {
            name: "p".to_string(),
            ty: Some(Type::Struct(ident("Point"), span())),
            kind: SymbolKind::Variable,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        let access = FieldAccess {
            object: Box::new(Expr::Variable(ident("p"), span())),
            field: ident("x"),
            span: span(),
        };

        let result = checker.check_field_access(&access);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Primitive(PrimitiveType::Int, _)));
    }

    #[test]
    fn test_field_access_nonexistent_field() {
        let mut checker = TypeChecker::new();

        let struct_def = StructDef {
            name: ident("Point"),
            generics: None,
            fields: vec![
                FieldDef {
                    name: ident("x"),
                    ty: int_type(),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_struct(&struct_def);

        // Register struct in struct_defs and symbol table
        checker.struct_defs.insert(struct_def.name.name.clone(), struct_def.clone());
        checker.symtab.insert(Symbol {
            name: struct_def.name.name.clone(),
            ty: None,
            span: struct_def.span,
            kind: SymbolKind::Type,
                    is_builtin: false,
        }).unwrap();

        checker.symtab.insert(Symbol {
            name: "p".to_string(),
            ty: Some(Type::Struct(ident("Point"), span())),
            kind: SymbolKind::Variable,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        // Access nonexistent field p.z
        let access = FieldAccess {
            object: Box::new(Expr::Variable(ident("p"), span())),
            field: ident("z"),
            span: span(),
        };

        checker.check_field_access(&access);
        assert!(!checker.errors.is_empty());
        assert!(matches!(checker.errors[0], TypeError::FieldNotFound { .. }));
    }

    // =============================================================================
    // Type Mismatch Tests
    // =============================================================================

    #[test]
    fn test_let_type_mismatch() {
        let mut checker = TypeChecker::new();

        // let x: Int = "hello";
        let let_stmt = LetStmt {
            mutable: false,
            name: ident("x"),
            ty: int_type(),
            init: Some(Box::new(Expr::Literal(Literal::String("hello".to_string()), span()))),
            span: span(),
        };

        checker.check_let(&let_stmt);
        assert!(!checker.errors.is_empty());
        assert!(matches!(checker.errors[0], TypeError::TypeMismatch { .. }));
    }

    // =============================================================================
    // Binary Operation Tests
    // =============================================================================

    #[test]
    fn test_binary_arithmetic_valid() {
        let mut checker = TypeChecker::new();

        let expr = BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::Int(1), span())),
            op: BinOp::Add,
            rhs: Box::new(Expr::Literal(Literal::Int(2), span())),
            span: span(),
        };

        let result = checker.check_binary(&expr);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Primitive(PrimitiveType::Int, _)));
    }

    #[test]
    fn test_binary_comparison_valid() {
        let mut checker = TypeChecker::new();

        let expr = BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::Int(1), span())),
            op: BinOp::Lt,
            rhs: Box::new(Expr::Literal(Literal::Int(2), span())),
            span: span(),
        };

        let result = checker.check_binary(&expr);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Primitive(PrimitiveType::Bool, _)));
    }

    #[test]
    fn test_binary_op_type_mismatch() {
        let mut checker = TypeChecker::new();

        // "hello" - 42 (subtraction not valid for strings)
        let expr = BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::String("hello".to_string()), span())),
            op: BinOp::Sub,
            rhs: Box::new(Expr::Literal(Literal::Int(42), span())),
            span: span(),
        };

        checker.check_binary(&expr);
        assert!(!checker.errors.is_empty());
    }

    #[test]
    fn test_string_concat_with_int() {
        let mut checker = TypeChecker::new();

        // "hello" + 42 → valid string concatenation
        let expr = BinaryExpr {
            lhs: Box::new(Expr::Literal(Literal::String("hello".to_string()), span())),
            op: BinOp::Add,
            rhs: Box::new(Expr::Literal(Literal::Int(42), span())),
            span: span(),
        };

        let result = checker.check_binary(&expr);
        assert!(checker.errors.is_empty());
        assert!(matches!(result, Some(Type::Primitive(PrimitiveType::String, _))));
    }

    // =============================================================================
    // Function Call Tests
    // =============================================================================

    #[test]
    fn test_function_call_valid() {
        let mut checker = TypeChecker::new();

        checker.symtab.insert(Symbol {
            name: "add".to_string(),
            ty: Some(Type::Function(FunctionType {
                params: vec![int_type(), int_type()],
                return_type: Box::new(int_type()),
                span: span(),
            })),
            kind: SymbolKind::Function,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        let call = CallExpr {
            func: Box::new(Expr::Variable(ident("add"), span())),
            args: vec![
                Expr::Literal(Literal::Int(1), span()),
                Expr::Literal(Literal::Int(2), span()),
            ],
            span: span(),
        };

        let result = checker.check_call(&call);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Primitive(PrimitiveType::Int, _)));
    }

    #[test]
    fn test_function_call_argument_count_mismatch() {
        let mut checker = TypeChecker::new();

        checker.symtab.insert(Symbol {
            name: "add".to_string(),
            ty: Some(Type::Function(FunctionType {
                params: vec![int_type(), int_type()],
                return_type: Box::new(int_type()),
                span: span(),
            })),
            kind: SymbolKind::Function,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        // Call with wrong argument count
        let call = CallExpr {
            func: Box::new(Expr::Variable(ident("add"), span())),
            args: vec![Expr::Literal(Literal::Int(1), span())],
            span: span(),
        };

        checker.check_call(&call);
        assert!(!checker.errors.is_empty());
        assert!(matches!(checker.errors[0], TypeError::ArgumentCountMismatch { .. }));
    }

    #[test]
    fn test_function_call_argument_type_mismatch() {
        let mut checker = TypeChecker::new();

        checker.symtab.insert(Symbol {
            name: "add".to_string(),
            ty: Some(Type::Function(FunctionType {
                params: vec![int_type(), int_type()],
                return_type: Box::new(int_type()),
                span: span(),
            })),
            kind: SymbolKind::Function,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        // Call with wrong argument type
        let call = CallExpr {
            func: Box::new(Expr::Variable(ident("add"), span())),
            args: vec![
                Expr::Literal(Literal::Int(1), span()),
                Expr::Literal(Literal::String("hello".to_string()), span()),
            ],
            span: span(),
        };

        checker.check_call(&call);
        assert!(!checker.errors.is_empty());
    }

    // =============================================================================
    // Array Tests
    // =============================================================================

    #[test]
    fn test_array_literal_homogeneous() {
        let mut checker = TypeChecker::new();

        let array = ArrayLiteral {
            elements: vec![
                Expr::Literal(Literal::Int(1), span()),
                Expr::Literal(Literal::Int(2), span()),
                Expr::Literal(Literal::Int(3), span()),
            ],
            span: span(),
        };

        let result = checker.check_array_literal(&array);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Array(_, _)));
    }

    #[test]
    fn test_array_literal_heterogeneous() {
        let mut checker = TypeChecker::new();

        // [1, "hello"] - heterogeneous
        let array = ArrayLiteral {
            elements: vec![
                Expr::Literal(Literal::Int(1), span()),
                Expr::Literal(Literal::String("hello".to_string()), span()),
            ],
            span: span(),
        };

        checker.check_array_literal(&array);
        assert!(!checker.errors.is_empty());
    }

    #[test]
    fn test_array_access_valid() {
        let mut checker = TypeChecker::new();

        checker.symtab.insert(Symbol {
            name: "arr".to_string(),
            ty: Some(Type::Array(Box::new(int_type()), span())),
            kind: SymbolKind::Variable,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        let access = ArrayAccess {
            array: Box::new(Expr::Variable(ident("arr"), span())),
            index: Box::new(Expr::Literal(Literal::Int(0), span())),
            span: span(),
        };

        let result = checker.check_array_access(&access);
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Type::Primitive(PrimitiveType::Int, _)));
    }

    #[test]
    fn test_array_access_non_integer_index() {
        let mut checker = TypeChecker::new();

        checker.symtab.insert(Symbol {
            name: "arr".to_string(),
            ty: Some(Type::Array(Box::new(int_type()), span())),
            kind: SymbolKind::Variable,
                    is_builtin: false,
            span: span(),
        }).unwrap();

        // arr["hello"] - invalid index
        let access = ArrayAccess {
            array: Box::new(Expr::Variable(ident("arr"), span())),
            index: Box::new(Expr::Literal(Literal::String("hello".to_string()), span())),
            span: span(),
        };

        checker.check_array_access(&access);
        assert!(!checker.errors.is_empty());
    }

    // =============================================================================
    // Match Expression Tests
    // =============================================================================

    #[test]
    fn test_match_consistent_arm_types() {
        let mut checker = TypeChecker::new();

        let match_expr = MatchExpr {
            expr: Box::new(Expr::Literal(Literal::Bool(true), span())),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(Literal::Bool(true), span()),
                    body: Box::new(Expr::Literal(Literal::Int(1), span())),
                    span: span(),
                },
                MatchArm {
                    pattern: Pattern::Literal(Literal::Bool(false), span()),
                    body: Box::new(Expr::Literal(Literal::Int(2), span())),
                    span: span(),
                },
            ],
            span: span(),
        };

        let result = checker.check_match(&match_expr);
        assert!(result.is_some());
    }

    #[test]
    fn test_match_inconsistent_arm_types() {
        let mut checker = TypeChecker::new();

        // Arms have different types
        let match_expr = MatchExpr {
            expr: Box::new(Expr::Literal(Literal::Bool(true), span())),
            arms: vec![
                MatchArm {
                    pattern: Pattern::Literal(Literal::Bool(true), span()),
                    body: Box::new(Expr::Literal(Literal::Int(1), span())),
                    span: span(),
                },
                MatchArm {
                    pattern: Pattern::Literal(Literal::Bool(false), span()),
                    body: Box::new(Expr::Literal(Literal::String("hello".to_string()), span())),
                    span: span(),
                },
            ],
            span: span(),
        };

        checker.check_match(&match_expr);
        assert!(!checker.errors.is_empty());
    }
}
