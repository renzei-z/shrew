use crate::{ HttpVersion, HttpStatus };

#[derive(Default)]
pub struct Response {
    version: HttpVersion,
    status: HttpStatus
}

impl Response {
    pub fn new() -> Response {
        Response::default()
    }

    pub fn as_http_response(&self) -> String {
        format!("HTTP/{} {} {}\r\n\r\n",
            self.version,
            self.status as isize,
            self.status
        )
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }
}