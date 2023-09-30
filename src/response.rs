use crate::server::Stream;

pub struct Response<'s> {
    stream: Stream<'s>,
    status: usize,
    headers: Vec<(String, String)>
}

impl<'r> Response<'r> {
    pub fn new(stream: Stream<'r>) -> Response<'r> {
        Response {
            stream,
            status: 200,
            headers: Vec::new()
        }
    }

    pub fn set_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    fn send_head(mut self) -> std::io::Result<Self> {
        self.stream.write(&format!("HTTP/1.1 {}\r\n", self.status))?;
        for (header_name, header_value) in &self.headers {
            self.stream.write(&format!("{}: {}\r\n", header_name, header_value))?;
        }
        self.stream.write("\r\n")?;

        Ok(self)
    }

    pub fn set_status(mut self, status: usize) -> Self {
        self.status = status;
        self
    }

    pub fn send_status(mut self, status: usize) -> std::io::Result<Self> {
        self = self.set_status(status).send_head()?;

        self.stream.close()?;
        Ok(self)
    }

    pub fn send(mut self, body: &str) -> std::io::Result<Self> {
        self = self
            .set_header("Content-Length", format!("{}", body.len()).as_str())
            .set_header("Content-Type", "text/plain; charset=utf-8")
            .send_head()?;
        self.stream.write(&body)?;
        
        self.stream.close()?;
        Ok(self)
    }
}