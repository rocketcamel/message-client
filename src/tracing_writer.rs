use std::{
    io,
    sync::{Arc, Mutex},
};

use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
pub struct BufferedWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl BufferedWriter {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_contents(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }
}

impl io::Write for BufferedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.lock().unwrap().flush()
    }
}

impl<'a> MakeWriter<'a> for BufferedWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
