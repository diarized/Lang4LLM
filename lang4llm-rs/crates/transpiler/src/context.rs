//! Code generation context — output buffer and indentation management.

use std::collections::HashMap;

use lang4llm_ast::Type;

/// Code generation context that accumulates output and tracks indentation.
pub struct CodegenContext {
    buf: String,
    indent_level: u32,
    /// Return types of user-defined functions, for string-concat detection.
    pub fn_return_types: HashMap<String, Type>,
}

impl CodegenContext {
    /// Create a new, empty context.
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            fn_return_types: HashMap::new(),
        }
    }

    /// Create a context pre-populated with function return type information.
    pub fn with_fn_types(fn_return_types: HashMap<String, Type>) -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            fn_return_types,
        }
    }

    /// Write a string slice directly to the buffer (no newline, no indent).
    pub fn write(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    /// Write a string followed by a newline.
    pub fn writeln(&mut self, s: &str) {
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    /// Increase indentation level by one step (4 spaces).
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease indentation level by one step (panics on underflow in debug).
    pub fn dedent(&mut self) {
        debug_assert!(self.indent_level > 0, "dedent called with indent_level == 0");
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Write a newline character.
    pub fn newline(&mut self) {
        self.buf.push('\n');
    }

    /// Write the current indentation prefix (spaces only, no newline).
    pub fn emit_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.buf.push_str("    ");
        }
    }

    /// Emit an indented line: indent + content + newline.
    pub fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.writeln(s);
    }

    /// Emit an indented string without a trailing newline.
    pub fn emit(&mut self, s: &str) {
        self.emit_indent();
        self.write(s);
    }

    /// Consume the context and return the accumulated output.
    pub fn finish(self) -> String {
        self.buf
    }
}

impl Default for CodegenContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_write() {
        let mut ctx = CodegenContext::new();
        ctx.write("hello");
        ctx.write(" world");
        assert_eq!(ctx.finish(), "hello world");
    }

    #[test]
    fn test_indent_dedent() {
        let mut ctx = CodegenContext::new();
        ctx.emit_line("fn foo() {");
        ctx.indent();
        ctx.emit_line("let x = 1;");
        ctx.dedent();
        ctx.emit_line("}");
        assert_eq!(ctx.finish(), "fn foo() {\n    let x = 1;\n}\n");
    }

    #[test]
    fn test_newline() {
        let mut ctx = CodegenContext::new();
        ctx.write("a");
        ctx.newline();
        ctx.write("b");
        assert_eq!(ctx.finish(), "a\nb");
    }

    #[test]
    fn test_nested_indent() {
        let mut ctx = CodegenContext::new();
        ctx.indent();
        ctx.indent();
        ctx.emit_indent();
        ctx.write("deep");
        assert_eq!(ctx.finish(), "        deep");
    }
}
