use std::fmt;

pub struct IndentFormatter<'a, 'b> {
    pub f: &'a mut fmt::Formatter<'b>,
    indent: usize,
    new_line: bool,
}

impl<'a, 'b> From<&'a mut fmt::Formatter<'b>> for IndentFormatter<'a, 'b> {
    fn from(f: &'a mut fmt::Formatter<'b>) -> Self {
        Self {
            f,
            indent: 0,
            new_line: false,
        }
    }
}

impl<'a, 'b> IndentFormatter<'a, 'b> {
    pub fn indent(&mut self) {
        self.indent += 1;
    }
    pub fn dedent(&mut self) {
        self.indent -= 1;
    }
    pub fn write(&mut self, d: impl fmt::Display) -> fmt::Result {
        if self.new_line {
            for _ in 0..self.indent {
                write!(self.f, "    ")?;
            }
            self.new_line = false;
        }
        write!(self.f, "{d}")
    }
    pub fn writeln(&mut self, d: impl fmt::Display) -> fmt::Result {
        self.new_line = true;
        writeln!(self.f, "{d}")
    }
}

pub trait IndentFormat {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> fmt::Result;
}
