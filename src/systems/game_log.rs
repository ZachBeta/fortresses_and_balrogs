use std::fs::File;
use std::io::{Write, BufWriter};

pub struct GameLog {
    pub writer: BufWriter<File>,
}

impl GameLog {
    pub fn new(filename: &str) -> std::io::Result<Self> {
        let file = File::create(filename)?;
        Ok(Self { writer: BufWriter::new(file) })
    }

    pub fn log_line(&mut self, line: &str) {
        let _ = writeln!(self.writer, "{}", line);
    }

    pub fn flush(&mut self) {
        let _ = self.writer.flush();
    }
}
