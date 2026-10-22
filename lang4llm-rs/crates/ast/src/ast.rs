//! AST node definitions for Lang4LLM
//!
//! This module contains all AST node types that represent Lang4LLM programs.
//! Each node corresponds to a production in grammar.ebnf.

use crate::Span;
use serde::{Deserialize, Serialize};

// =============================================================================
// Top-Level Program
// =============================================================================

/// A complete Lang4LLM program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// Module declaration (optional)
    pub module: Option<ModuleDecl>,
    /// Import statements
    pub imports: Vec<ImportStmt>,
    /// Top-level items (functions, structs, enums, traits, impls)
    pub items: Vec<Item>,
    /// Source span
    pub span: Span,
}

/// Module declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleDecl {
    /// Module name
    pub name: Ident,
    /// Source span
    pub span: Span,
}

/// Import statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStmt {
    /// Imported path (e.g., `std.io`)
    pub path: Vec<Ident>,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Top-Level Items
// =============================================================================

/// Top-level item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    /// Function definition
    Function(FunctionDef),
    /// Struct definition
    Struct(StructDef),
    /// Enum definition
    Enum(EnumDef),
    /// Trait definition
    Trait(TraitDef),
    /// Implementation block
    Impl(ImplBlock),
}

impl Item {
    /// Get the span of this item
    pub fn span(&self) -> Span {
        match self {
            Item::Function(f) => f.span,
            Item::Struct(s) => s.span,
            Item::Enum(e) => e.span,
            Item::Trait(t) => t.span,
            Item::Impl(i) => i.span,
        }
    }
}

/// Function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDef {
    /// Function name
    pub name: Ident,
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Function parameters
    pub params: Vec<Param>,
    /// Return type
    pub return_type: Type,
    /// Function body
    pub body: BlockExpr,
    /// Source span
    pub span: Span,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name (`self` or identifier)
    pub name: ParamName,
    /// Parameter type
    pub ty: Type,
    /// Source span
    pub span: Span,
}

/// Parameter name
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParamName {
    /// `self` parameter
    SelfParam,
    /// Regular identifier
    Ident(Ident),
    /// Generic parameter with trait bounds
    GenericBound(Ident, Vec<Ident>),
}

/// Single generic parameter with optional trait bound
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParamDef {
    /// Parameter name (e.g. `T`)
    pub name: Ident,
    /// Optional trait bound (e.g. `Display` in `T: Display`)
    pub bound: Option<Ident>,
}

/// Generic type parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParams {
    /// Type parameter definitions (name + optional bound)
    pub params: Vec<GenericParamDef>,
    /// Source span
    pub span: Span,
}

/// Struct definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    /// Struct name
    pub name: Ident,
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Struct fields
    pub fields: Vec<FieldDef>,
    /// Source span
    pub span: Span,
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDef {
    /// Field name
    pub name: Ident,
    /// Field type
    pub ty: Type,
    /// Source span
    pub span: Span,
}

/// Enum definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDef {
    /// Enum name
    pub name: Ident,
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Enum variants
    pub variants: Vec<Variant>,
    /// Source span
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// Variant name
    pub name: Ident,
    /// Associated data types (for tuple variants)
    pub data: Option<Vec<Type>>,
    /// Source span
    pub span: Span,
}

/// Trait definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitDef {
    /// Trait name
    pub name: Ident,
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Trait items (function signatures)
    pub items: Vec<FunctionSig>,
    /// Source span
    pub span: Span,
}

/// Function signature (in traits)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionSig {
    /// Function name
    pub name: Ident,
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Parameters
    pub params: Vec<Param>,
    /// Return type
    pub return_type: Type,
    /// Source span
    pub span: Span,
}

/// Implementation block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplBlock {
    /// Generic type parameters (optional)
    pub generics: Option<GenericParams>,
    /// Trait being implemented (optional, for trait impls)
    pub trait_name: Option<TraitName>,
    /// Type being implemented for
    pub type_name: TypeName,
    /// Implementation items (methods)
    pub items: Vec<FunctionDef>,
    /// Source span
    pub span: Span,
}

/// Trait name with optional generic arguments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitName {
    /// Trait identifier
    pub name: Ident,
    /// Generic arguments (optional)
    pub generics: Option<GenericArgs>,
    /// Source span
    pub span: Span,
}

/// Type name
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeName {
    /// Type identifier
    pub name: Ident,
    /// Generic arguments (optional)
    pub generics: Option<GenericArgs>,
    /// Source span
    pub span: Span,
}

/// Generic type arguments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericArgs {
    /// Type arguments
    pub args: Vec<Type>,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Statements
// =============================================================================

/// Statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    /// Let binding
    Let(LetStmt),
    /// Assignment
    Assign(AssignStmt),
    /// Expression statement
    Expr(ExprStmt),
    /// Return statement
    Return(ReturnStmt),
    /// Break statement
    Break(BreakStmt),
    /// Continue statement
    Continue(ContinueStmt),
    /// While loop
    While(WhileStmt),
    /// For loop
    For(ForStmt),
    /// Infinite loop
    Loop(LoopStmt),
    /// Empty statement (`;`)
    Empty(Span),
}

impl Stmt {
    /// Get the span of this statement
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let(s) => s.span,
            Stmt::Assign(s) => s.span,
            Stmt::Expr(s) => s.span,
            Stmt::Return(s) => s.span,
            Stmt::Break(s) => s.span,
            Stmt::Continue(s) => s.span,
            Stmt::While(s) => s.span,
            Stmt::For(s) => s.span,
            Stmt::Loop(s) => s.span,
            Stmt::Empty(s) => *s,
        }
    }
}

/// Let statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStmt {
    /// Mutable?
    pub mutable: bool,
    /// Variable name
    pub name: Ident,
    /// Type annotation
    pub ty: Type,
    /// Initializer expression (None for uninitialized declarations)
    pub init: Option<Box<Expr>>,
    /// Source span
    pub span: Span,
}

/// Assignment statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignStmt {
    /// Left-hand side (variable, array access, field access, or tuple pattern)
    pub lhs: AssignTarget,
    /// Right-hand side expression
    pub rhs: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Assignment target
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssignTarget {
    /// Variable
    Var(Ident),
    /// Array access
    ArrayAccess(Box<Expr>, Box<Expr>, Span),
    /// Field access
    FieldAccess(Box<Expr>, Ident, Span),
    /// Tuple pattern
    Tuple(Vec<Pattern>, Span),
}

/// Expression statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExprStmt {
    /// Expression
    pub expr: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Return statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStmt {
    /// Return value (optional)
    pub value: Option<Box<Expr>>,
    /// Source span
    pub span: Span,
}

/// Break statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakStmt {
    /// Source span
    pub span: Span,
}

/// Continue statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueStmt {
    /// Source span
    pub span: Span,
}

/// While loop
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStmt {
    /// Condition
    pub condition: Box<Expr>,
    /// Body
    pub body: BlockExpr,
    /// Source span
    pub span: Span,
}

/// For loop
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStmt {
    /// Loop variable
    pub var: Ident,
    /// Iterator expression
    pub iter: Box<Expr>,
    /// Body
    pub body: BlockExpr,
    /// Source span
    pub span: Span,
}

/// Infinite loop
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopStmt {
    /// Body
    pub body: BlockExpr,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Expressions
// =============================================================================

/// Expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Literal
    Literal(Literal, Span),
    /// Variable
    Variable(Ident, Span),
    /// Binary operation
    Binary(BinaryExpr),
    /// Unary operation
    Unary(UnaryExpr),
    /// Function call
    Call(CallExpr),
    /// Array literal
    Array(ArrayLiteral),
    /// Tuple literal
    Tuple(TupleLiteral),
    /// Struct literal
    Struct(StructLiteral),
    /// Array access/indexing
    ArrayAccess(ArrayAccess),
    /// Field access
    FieldAccess(FieldAccess),
    /// Parenthesized expression
    Paren(Box<Expr>, Span),
    /// If-else expression
    If(IfExpr),
    /// Match expression
    Match(MatchExpr),
    /// Block expression
    Block(BlockExpr),
    /// Range expression
    Range(RangeExpr),
    /// Closure/lambda
    Closure(ClosureExpr),
    /// Enum variant literal
    EnumVariant(EnumVariantLiteral),
}

impl Expr {
    /// Get the span of this expression
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s) => *s,
            Expr::Variable(_, s) => *s,
            Expr::Binary(e) => e.span,
            Expr::Unary(e) => e.span,
            Expr::Call(e) => e.span,
            Expr::Array(e) => e.span,
            Expr::Tuple(e) => e.span,
            Expr::Struct(e) => e.span,
            Expr::ArrayAccess(e) => e.span,
            Expr::FieldAccess(e) => e.span,
            Expr::Paren(_, s) => *s,
            Expr::If(e) => e.span,
            Expr::Match(e) => e.span,
            Expr::Block(e) => e.span,
            Expr::Range(e) => e.span,
            Expr::Closure(e) => e.span,
            Expr::EnumVariant(e) => e.span,
        }
    }
}

/// Literal value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Integer literal
    Int(i64),
    /// Float literal
    Float(f64),
    /// Boolean literal
    Bool(bool),
    /// String literal
    String(String),
    /// None literal (unit value)
    None,
}

/// Binary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
    /// Left-hand side
    pub lhs: Box<Expr>,
    /// Operator
    pub op: BinOp,
    /// Right-hand side
    pub rhs: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    /// Addition (`+`), also string concatenation
    Add,
    /// Subtraction (`-`)
    Sub,
    /// Multiplication (`*`)
    Mul,
    /// Division (`/`)
    Div,
    /// Modulo (`%`)
    Mod,
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
    /// Less than (`<`)
    Lt,
    /// Less than or equal (`<=`)
    Le,
    /// Greater than (`>`)
    Gt,
    /// Greater than or equal (`>=`)
    Ge,
    /// Logical AND (`and`)
    And,
    /// Logical OR (`or`)
    Or,
}

/// Unary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    /// Operator
    pub op: UnOp,
    /// Operand
    pub operand: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    /// Logical NOT (`not`)
    Not,
    /// Arithmetic negation (`-`)
    Neg,
}

/// Function call
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
    /// Function expression (identifier, field access, or any callable expression)
    pub func: Box<Expr>,
    /// Arguments
    pub args: Vec<Expr>,
    /// Source span
    pub span: Span,
}

/// Array literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayLiteral {
    /// Elements
    pub elements: Vec<Expr>,
    /// Source span
    pub span: Span,
}

/// Tuple literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleLiteral {
    /// Elements (at least 2)
    pub elements: Vec<Expr>,
    /// Source span
    pub span: Span,
}

/// Struct literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructLiteral {
    /// Struct name
    pub name: Ident,
    /// Field initializers
    pub fields: Vec<FieldInit>,
    /// Source span
    pub span: Span,
}

/// Field initializer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldInit {
    /// Field name
    pub name: Ident,
    /// Field value
    pub value: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Array access
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayAccess {
    /// Array expression
    pub array: Box<Expr>,
    /// Index expression
    pub index: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Field access
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccess {
    /// Object expression
    pub object: Box<Expr>,
    /// Field name
    pub field: Ident,
    /// Source span
    pub span: Span,
}

/// If-else expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
    /// Condition
    pub condition: Box<Expr>,
    /// Then branch
    pub then_branch: BlockExpr,
    /// Else branch (optional - if without else is a statement-level construct)
    pub else_branch: Option<BlockExpr>,
    /// Source span
    pub span: Span,
}

/// Match expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchExpr {
    /// Expression to match on
    pub expr: Box<Expr>,
    /// Match arms
    pub arms: Vec<MatchArm>,
    /// Source span
    pub span: Span,
}

/// Match arm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    /// Pattern
    pub pattern: Pattern,
    /// Arm body
    pub body: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Block expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockExpr {
    /// Statements
    pub stmts: Vec<Stmt>,
    /// Final expression (optional)
    pub expr: Option<Box<Expr>>,
    /// Source span
    pub span: Span,
}

/// Range expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeExpr {
    /// Start (optional)
    pub start: Option<Box<Expr>>,
    /// Operator (.. or ..=)
    pub op: RangeOp,
    /// End (optional)
    pub end: Option<Box<Expr>>,
    /// Source span
    pub span: Span,
}

/// Range operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeOp {
    /// Exclusive range (`..`)
    Exclusive,
    /// Inclusive range (`..=`)
    Inclusive,
}

/// Closure expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosureExpr {
    /// Parameters
    pub params: Vec<Param>,
    /// Body expression
    pub body: Box<Expr>,
    /// Source span
    pub span: Span,
}

/// Enum variant literal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariantLiteral {
    /// Enum name
    pub enum_name: Ident,
    /// Variant name
    pub variant_name: Ident,
    /// Arguments (optional, for tuple variants)
    pub args: Option<Vec<Expr>>,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Patterns
// =============================================================================

/// Pattern (for match expressions and destructuring)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    /// Literal pattern
    Literal(Literal, Span),
    /// Identifier pattern (binds variable)
    Ident(Ident, Span),
    /// Wildcard pattern (`_`)
    Wildcard(Span),
    /// Tuple pattern
    Tuple(Vec<Pattern>, Span),
    /// Struct pattern
    Struct(StructPattern),
    /// Enum pattern
    Enum(EnumPattern),
}

impl Pattern {
    /// Get the span of this pattern
    pub fn span(&self) -> Span {
        match self {
            Pattern::Literal(_, s) => *s,
            Pattern::Ident(_, s) => *s,
            Pattern::Wildcard(s) => *s,
            Pattern::Tuple(_, s) => *s,
            Pattern::Struct(p) => p.span,
            Pattern::Enum(p) => p.span,
        }
    }
}

/// Struct pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructPattern {
    /// Struct name
    pub name: Ident,
    /// Field patterns
    pub fields: Vec<FieldPattern>,
    /// Source span
    pub span: Span,
}

/// Field pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldPattern {
    /// Field name
    pub name: Ident,
    /// Field pattern
    pub pattern: Box<Pattern>,
    /// Source span
    pub span: Span,
}

/// Enum pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumPattern {
    /// Enum name
    pub enum_name: Option<Ident>,
    /// Variant name
    pub variant_name: Ident,
    /// Associated data patterns (optional)
    pub patterns: Option<Vec<Pattern>>,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Types
// =============================================================================

/// Type annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Primitive type
    Primitive(PrimitiveType, Span),
    /// Array type
    Array(Box<Type>, Span),
    /// Tuple type
    Tuple(Vec<Type>, Span),
    /// Generic type
    Generic(Ident, Option<GenericArgs>, Span),
    /// Struct type (identifier)
    Struct(Ident, Span),
    /// Enum type (identifier)
    Enum(Ident, Span),
    /// Function type
    Function(FunctionType),
    /// Inferred type (closure params without annotation)
    Inferred(Span),
}

impl Type {
    /// Get the span of this type
    pub fn span(&self) -> Span {
        match self {
            Type::Primitive(_, s) => *s,
            Type::Array(_, s) => *s,
            Type::Tuple(_, s) => *s,
            Type::Generic(_, _, s) => *s,
            Type::Struct(_, s) => *s,
            Type::Enum(_, s) => *s,
            Type::Function(t) => t.span,
            Type::Inferred(s) => *s,
        }
    }
}

/// Primitive type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    /// Integer type
    Int,
    /// Float type
    Float,
    /// Boolean type
    Bool,
    /// String type
    String,
    /// Unit type
    Unit,
}

/// Function type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionType {
    /// Parameter types
    pub params: Vec<Type>,
    /// Return type
    pub return_type: Box<Type>,
    /// Source span
    pub span: Span,
}

// =============================================================================
// Identifiers
// =============================================================================

/// Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ident {
    /// Identifier name
    pub name: String,
    /// Source span
    pub span: Span,
}

impl Ident {
    /// Create a new identifier
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}
