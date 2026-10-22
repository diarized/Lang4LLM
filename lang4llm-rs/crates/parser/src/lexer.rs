//! Lexer for Lang4LLM
//!
//! Converts source code into a stream of tokens.
//! Handles keywords, identifiers, literals, operators, and comments.

use lang4llm_ast::Span;
use std::fmt;

/// A token in the Lang4LLM language
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// Source location of this token
    pub span: Span,
}

/// Token kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let,
    Mut,
    Fn,
    Return,
    If,
    Else,
    Match,
    For,
    In,
    While,
    Loop,
    Break,
    Continue,
    Struct,
    Enum,
    Impl,
    Trait,
    Import,
    Export,
    Module,
    True,
    False,
    None,

    // Type keywords
    Int,
    Float,
    Bool,
    String,
    Unit,

    // Logical operators (keywords)
    And,
    Or,
    Not,

    // Other keywords
    As,
    Is,
    SelfKeyword,
    Underscore, // _ (wildcard pattern)

    // Identifiers and literals
    Ident((String, Span)),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),

    // Operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Eq,      // ==
    Ne,      // !=
    Lt,      // <
    Le,      // <=
    Gt,      // >
    Ge,      // >=
    Assign,  // =

    // Range operators
    Range,   // ..
    RangeEq, // ..=

    // Delimiters
    LBrace,     // {
    RBrace,     // }
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    Colon,      // :
    ColonColon, // ::
    Comma,      // ,
    Semi,       // ;
    Dot,        // .
    Pipe,       // |
    Arrow,      // ->
    FatArrow,   // =>

    // Special
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::For => write!(f, "for"),
            TokenKind::In => write!(f, "in"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Impl => write!(f, "impl"),
            TokenKind::Trait => write!(f, "trait"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::Module => write!(f, "module"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::None => write!(f, "none"),
            TokenKind::Int => write!(f, "int"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Bool => write!(f, "bool"),
            TokenKind::String => write!(f, "string"),
            TokenKind::Unit => write!(f, "unit"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::As => write!(f, "as"),
            TokenKind::Is => write!(f, "is"),
            TokenKind::SelfKeyword => write!(f, "self"),
            TokenKind::Underscore => write!(f, "_"),
            TokenKind::Ident((s, _)) => write!(f, "{}", s),
            TokenKind::IntLit(n) => write!(f, "{}", n),
            TokenKind::FloatLit(fl) => write!(f, "{}", fl),
            TokenKind::StringLit(s) => write!(f, "\"{}\"", s),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Eq => write!(f, "=="),
            TokenKind::Ne => write!(f, "!="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Le => write!(f, "<="),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Ge => write!(f, ">="),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Range => write!(f, ".."),
            TokenKind::RangeEq => write!(f, "..="),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

/// Lexer for Lang4LLM source code
pub struct Lexer<'a> {
    /// Source code
    source: &'a str,
    /// Current position in bytes
    pos: usize,
    /// Current line (1-indexed)
    line: usize,
    /// Current column (1-indexed)
    col: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Peek at the current character without consuming it
    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Peek at the next character without consuming
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.pos..].chars();
        chars.next();
        chars.next()
    }

    /// Consume and return the current character
    fn consume(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    /// Skip line comment (// ...)
    fn skip_line_comment(&mut self) {
        // Consume //
        self.consume();
        self.consume();

        // Skip until newline
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.consume();
        }
    }

    /// Skip block comment (/* ... */)
    fn skip_block_comment(&mut self) -> Result<(), String> {
        // Consume /*
        self.consume();
        self.consume();

        // Skip until */
        loop {
            match self.peek() {
                None => return Err("Unterminated block comment".to_string()),
                Some('*') if self.peek_next() == Some('/') => {
                    self.consume(); // *
                    self.consume(); // /
                    break;
                }
                Some(_) => {
                    self.consume();
                }
            }
        }
        Ok(())
    }

    /// Lex an identifier or keyword
    fn lex_ident_or_keyword(&mut self, start_line: usize, start_col: usize) -> Token {
        let mut ident = String::new();

        // First character must be letter or underscore
        if let Some(ch) = self.consume() {
            ident.push(ch);
        }

        // Remaining characters: letter, digit, or underscore
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.consume();
            } else {
                break;
            }
        }

        let span = Span::new(start_line, start_col, self.line, self.col);

        // Check if it's a keyword or underscore wildcard
        let kind = match ident.as_str() {
            "_" => TokenKind::Underscore,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "impl" => TokenKind::Impl,
            "trait" => TokenKind::Trait,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "module" => TokenKind::Module,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "none" => TokenKind::None,
            "int" => TokenKind::Int,
            "float" => TokenKind::Float,
            "bool" => TokenKind::Bool,
            "string" => TokenKind::String,
            "unit" => TokenKind::Unit,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "as" => TokenKind::As,
            "is" => TokenKind::Is,
            "self" => TokenKind::SelfKeyword,
            _ => TokenKind::Ident((ident, span)),
        };

        Token { kind, span }
    }

    /// Lex a number literal (integer or float)
    fn lex_number(&mut self, start_line: usize, start_col: usize) -> Result<Token, String> {
        let mut num_str = String::new();
        let mut is_float = false;

        // Consume digits
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.consume();
            } else if ch == '.'
                && !is_float
                && self
                    .peek_next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
            {
                // It's a float literal
                is_float = true;
                num_str.push(ch);
                self.consume();
            } else {
                break;
            }
        }

        let span = Span::new(start_line, start_col, self.line, self.col);

        let kind = if is_float {
            let f = num_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid float literal: {}", num_str))?;
            TokenKind::FloatLit(f)
        } else {
            let n = num_str
                .parse::<i64>()
                .map_err(|_| format!("Invalid integer literal: {}", num_str))?;
            TokenKind::IntLit(n)
        };

        Ok(Token { kind, span })
    }

    /// Lex a string literal
    fn lex_string(&mut self, start_line: usize, start_col: usize) -> Result<Token, String> {
        // Consume opening quote
        self.consume();

        let mut string_val = String::new();

        loop {
            match self.peek() {
                None | Some('\n') => {
                    return Err("Unterminated string literal".to_string());
                }
                Some('"') => {
                    self.consume();
                    break;
                }
                Some('\\') => {
                    self.consume();
                    // Handle escape sequences
                    match self.peek() {
                        Some('n') => {
                            string_val.push('\n');
                            self.consume();
                        }
                        Some('t') => {
                            string_val.push('\t');
                            self.consume();
                        }
                        Some('r') => {
                            string_val.push('\r');
                            self.consume();
                        }
                        Some('"') => {
                            string_val.push('"');
                            self.consume();
                        }
                        Some('\\') => {
                            string_val.push('\\');
                            self.consume();
                        }
                        Some(ch) => {
                            return Err(format!("Invalid escape sequence: \\{}", ch));
                        }
                        None => {
                            return Err("Unterminated escape sequence".to_string());
                        }
                    }
                }
                Some(ch) => {
                    string_val.push(ch);
                    self.consume();
                }
            }
        }

        let span = Span::new(start_line, start_col, self.line, self.col);
        Ok(Token {
            kind: TokenKind::StringLit(string_val),
            span,
        })
    }

    /// Get the next token
    /// Skip all whitespace and comments, positioning at next meaningful token
    fn skip_trivia(&mut self) -> Result<(), String> {
        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('/') if self.peek_next() == Some('/') => {
                    self.skip_line_comment();
                }
                Some('/') if self.peek_next() == Some('*') => {
                    self.skip_block_comment()?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_trivia()?;

        let start_line = self.line;
        let start_col = self.col;

        match self.peek() {
            None => Ok(Token {
                kind: TokenKind::Eof,
                span: Span::new(self.line, self.col, self.line, self.col),
            }),

            // Identifiers and keywords (including underscore)
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
                Ok(self.lex_ident_or_keyword(start_line, start_col))
            }

            // Numbers
            Some(ch) if ch.is_ascii_digit() => self.lex_number(start_line, start_col),

            // String literals
            Some('"') => self.lex_string(start_line, start_col),

            // Operators and delimiters
            Some('=') => {
                self.consume();
                if self.peek() == Some('=') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::Eq,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else if self.peek() == Some('>') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::FatArrow,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Assign,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some('!') => {
                self.consume();
                if self.peek() == Some('=') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::Ne,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Err("Unexpected character: ! (did you mean '!='?)".to_string())
                }
            }

            Some('<') => {
                self.consume();
                if self.peek() == Some('=') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::Le,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Lt,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some('>') => {
                self.consume();
                if self.peek() == Some('=') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::Ge,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Gt,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some('.') => {
                self.consume();
                if self.peek() == Some('.') {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Ok(Token {
                            kind: TokenKind::RangeEq,
                            span: Span::new(start_line, start_col, self.line, self.col),
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Range,
                            span: Span::new(start_line, start_col, self.line, self.col),
                        })
                    }
                } else {
                    // Single dot for member access
                    Ok(Token {
                        kind: TokenKind::Dot,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some('-') => {
                self.consume();
                if self.peek() == Some('>') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::Arrow,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Minus,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some('+') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Plus,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('*') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Star,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('/') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Slash,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('%') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Percent,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('{') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::LBrace,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('}') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::RBrace,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('(') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::LParen,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some(')') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::RParen,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('[') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::LBracket,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some(']') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::RBracket,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some(':') => {
                self.consume();
                if self.peek() == Some(':') {
                    self.consume();
                    Ok(Token {
                        kind: TokenKind::ColonColon,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Colon,
                        span: Span::new(start_line, start_col, self.line, self.col),
                    })
                }
            }

            Some(',') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Comma,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some(';') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Semi,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some('|') => {
                self.consume();
                Ok(Token {
                    kind: TokenKind::Pipe,
                    span: Span::new(start_line, start_col, self.line, self.col),
                })
            }

            Some(ch) => Err(format!("Unexpected character: {}", ch)),
        }
    }

    /// Check if the lexer has reached the end of input
    pub fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Tokenize the entire source into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

/// Iterator adapter for LALRPOP integration.
/// Yields `(start_pos, TokenKind, end_pos)` triples as required by LALRPOP.
/// Positions are byte offsets (matching LALRPOP's `type Location = usize`).
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, TokenKind, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace/comments first, then record position
        if let Err(e) = self.skip_trivia() {
            return Some(Err(e));
        }
        let start = self.pos;
        match self.next_token() {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    None
                } else {
                    let end = self.pos;
                    Some(Ok((start, token.kind, end)))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let mut fn return");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Mut);
        assert_eq!(tokens[2].kind, TokenKind::Fn);
        assert_eq!(tokens[3].kind, TokenKind::Return);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("foo bar_baz x123");
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(&tokens[0].kind, TokenKind::Ident((s, _)) if s == "foo"));
        assert!(matches!(&tokens[1].kind, TokenKind::Ident((s, _)) if s == "bar_baz"));
        assert!(matches!(&tokens[2].kind, TokenKind::Ident((s, _)) if s == "x123"));
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::IntLit(42));
        assert_eq!(tokens[1].kind, TokenKind::FloatLit(3.14));
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" "world\n""#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::StringLit("hello".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::StringLit("world\n".to_string()));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / % == != < <= > >= =");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
        assert_eq!(tokens[4].kind, TokenKind::Percent);
        assert_eq!(tokens[5].kind, TokenKind::Eq);
        assert_eq!(tokens[6].kind, TokenKind::Ne);
        assert_eq!(tokens[7].kind, TokenKind::Lt);
        assert_eq!(tokens[8].kind, TokenKind::Le);
        assert_eq!(tokens[9].kind, TokenKind::Gt);
        assert_eq!(tokens[10].kind, TokenKind::Ge);
        assert_eq!(tokens[11].kind, TokenKind::Assign);
    }

    #[test]
    fn test_delimiters() {
        let mut lexer = Lexer::new("{ } ( ) [ ] : :: , ; . -> =>");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LBrace);
        assert_eq!(tokens[1].kind, TokenKind::RBrace);
        assert_eq!(tokens[2].kind, TokenKind::LParen);
        assert_eq!(tokens[3].kind, TokenKind::RParen);
        assert_eq!(tokens[4].kind, TokenKind::LBracket);
        assert_eq!(tokens[5].kind, TokenKind::RBracket);
        assert_eq!(tokens[6].kind, TokenKind::Colon);
        assert_eq!(tokens[7].kind, TokenKind::ColonColon);
        assert_eq!(tokens[8].kind, TokenKind::Comma);
        assert_eq!(tokens[9].kind, TokenKind::Semi);
        assert_eq!(tokens[10].kind, TokenKind::Dot);
        assert_eq!(tokens[11].kind, TokenKind::Arrow);
        assert_eq!(tokens[12].kind, TokenKind::FatArrow);
    }

    #[test]
    fn test_comments() {
        let source = r#"
            // line comment
            let x: int = 42; /* block comment */
            let y: int = 10;
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        // Should skip comments
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert!(matches!(&tokens[1].kind, TokenKind::Ident((s, _)) if s == "x"));
    }

    #[test]
    fn test_simple_function() {
        let source = r#"
            fn add(a: int, b: int) -> int {
                return a + b;
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert!(matches!(&tokens[1].kind, TokenKind::Ident((s, _)) if s == "add"));
        assert_eq!(tokens[2].kind, TokenKind::LParen);
        assert!(matches!(&tokens[3].kind, TokenKind::Ident((s, _)) if s == "a"));
        assert_eq!(tokens[4].kind, TokenKind::Colon);
        assert_eq!(tokens[5].kind, TokenKind::Int);
    }
}
