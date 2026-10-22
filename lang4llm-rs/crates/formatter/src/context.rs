//! Formatting context — output buffer + indentation control.

/// Formatting context: accumulates output and tracks indentation level.
#[derive(Default)]
pub struct FmtContext {
    buf: String,
    indent: usize,
}

impl FmtContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Write text without a trailing newline.
    pub fn write(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    /// Write text followed by a newline.
    pub fn writeln(&mut self, s: &str) {
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    /// Append a bare newline.
    pub fn newline(&mut self) {
        self.buf.push('\n');
    }

    /// Emit the current indentation (4 spaces per level).
    pub fn emit_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str("    ");
        }
    }

    /// Emit indentation + text + newline.
    pub fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    /// Increase indentation level.
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease indentation level.
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    /// Take the accumulated output.
    pub fn finish(self) -> String {
        self.buf
    }
}
