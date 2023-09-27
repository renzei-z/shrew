use crate::{ HttpVersion, HttpStatus, stream::Stream };

pub struct Response<'r> {
    stream: &'r mut Stream<'r>,
    headers: Vec<(String, String)>,
    version: HttpVersion,
    status: HttpStatus
}

impl Response<'_> {
    pub fn new<'r>(stream: &'r mut Stream<'r>) -> Response<'r> {
        Response {
            stream,
            headers: Vec::<(String, String)>::new(),
            version: HttpVersion::OnePointOne,
            status: HttpStatus::OK
        }
    }

    pub fn send_status(&mut self, status: HttpStatus) -> std::io::Result<()> {
        self.status = status;

        self.headers.push(("Location".to_string(), "https://google.se".to_string()));

        self.send_no_body()?;
        
        Ok(())
    }

    fn send_no_body(&mut self) -> std::io::Result<()> {
        self.stream.write("HTTP/1.1 ")?;
        self.stream.write(&format!("{} {}\r\n", self.status as isize, self.status))?;

        for (name, val) in self.headers.iter() {
            self.stream.write(&format!("{}: {}", name, val))?;
        }

        self.stream.write(&format!("\r\n\r\n{}\r\n\r\n", self.status as isize))?;

        Ok(())
        //self.end()
    }

    pub fn end(&mut self) -> std::io::Result<()> {
        match self.stream.close() {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Couldn't close connection: {}", e.to_string());
                Ok(())
            }
        }
    }
}