use std::fmt;

pub enum RequestMethod {
    GET,
    OPTIONS,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT
}

pub enum RequestError  {
    UnknownMethod(String)
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::CONNECT => "CONNECT",
            RequestMethod::TRACE => "TRACE",
            RequestMethod::DELETE => "DELETE",
            RequestMethod::PUT => "PUT",
            RequestMethod::HEAD => "HEAD",
            RequestMethod::OPTIONS => "OPTIONS"
        })
    }
}

pub struct Request {
    method: Option<RequestMethod>
}

impl Request {
    fn new() -> Request {
        Request {
            method: None
        }
    }

    pub fn from_string(_string: String) -> Result<Request, RequestError> {
        let request = Request::new();

        Ok(request)
    }

    pub fn pretty_print(&self) {
        println!("Request: {{");
        println!("\tMethod: {}", match &self.method {
            Some(m) => format!("{}", m),
            None => "Unknown".to_string()
        });
        println!("}}");
    }
}