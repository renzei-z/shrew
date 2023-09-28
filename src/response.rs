use crate::stream::Stream;

#[allow(dead_code)]
pub struct Response<'r> {
    stream: Stream<'r>,
    headers: Vec<(String, String)>,
    status_code: usize,
    finished: bool
}

impl<'s> Response<'s> {
    // TODO: Make this crate public only
    pub fn new(stream: Stream<'s>) -> Self { 
        Response {
            stream,
            headers: Vec::new(),
            status_code: 200,
            finished: false
        }
    }
}

// These functions are those accessible
impl<'r> Response<'r> {
    pub fn set_header(mut self, header: &str, value: &str) -> Self {
        self.headers.push((header.to_string(), value.to_string()));
        self
    }
    
    pub fn send_empty(mut self) -> std::io::Result<()> {
        self.stream.write(&format!("HTTP/1.1 {}\r\n", self.status_code))?;

        for (header, value) in self.headers {
            self.stream.write(&format!("{}: {}\r\n", header, value))?;
        }

        self.stream.write("\r\n")?;

        self.stream.close()?;

        self.finished = true;

        Ok(())
    }

    pub fn send_status(mut self, status: usize) -> std::io::Result<()> {
        // TODO: Validate status code
        self.status_code = status; 
        self.send_empty()
    }
}
