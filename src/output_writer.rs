use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub trait OutputWriter {
    fn write(&self, output: &str);
}

pub struct StringWriter {
    output: Mutex<String>,
}

impl StringWriter {
    pub fn new() -> Self {
        StringWriter {
            output: Mutex::new(String::new()),
        }
    }
}

impl OutputWriter for StringWriter {
    fn write(&self, output: &str) {
        self.output.lock().unwrap().push_str(output);
    }
}

pub struct IndentWriter<'a, W: OutputWriter> {
    indent: u32,
    writer: &'a W,
}

impl<W: OutputWriter> OutputWriter for IndentWriter<'_, W> {
    fn write(&self, output: &str) {
        for _ in 0..self.indent {
            self.writer.write(" ");
        }

        self.writer.write(output);
    }
}

pub struct A {
    pub b_vec: Vec<B>,
}

impl A {
    pub fn format<W: OutputWriter>(&self, writer: &W) {
        writer.write("MATCH\n");
        let inner_writer = IndentWriter { indent: 2, writer };
        self.b_vec.iter().for_each(|b| b.format(&inner_writer));
    }
}

pub struct B;

impl B {
    pub fn format<W: OutputWriter>(&self, writer: &W) {
        writer.write("(a:Node)-[:SomeRel]-(b:Node)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut writer = StringWriter::new();
        let b0 = B;
        let b1 = B;

        let a = A {
            //b_vec: vec![b0, b1],
            b_vec: vec![b0]
        };

        a.format(&writer);
        println!("{}", writer.output.get_mut().unwrap());
    }

    #[test]
    fn output_with_string_writer() {
        let mut writer = StringWriter::new();
        writer.write("abc");
        assert_eq!("abc", writer.output.get_mut().unwrap());
    }
}
