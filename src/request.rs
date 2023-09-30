use regex::Regex;
use std::collections::HashMap;

type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct Request {
    _method: String,
    pub request_uri: String,
    _headers: Headers
}

// Request structure (as per section 5 of RFC 2616)
// https://www.rfc-editor.org/rfc/rfc2616#section-5
//
// Request-Line
//        -> Method SP Request-URI SP HTTP-Version CRLF
// *(( general-header 
//      | request-header
//      | entity-header ) CRLF )
// CRLF
// [message-body] <- Ignored for now.

impl TryFrom<String> for Request {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO: Don't compile regexes on every request.
        let request_line_re: Regex = Regex::new(r"([A-Z]+) (\/[^\s]*) HTTP\/(1|1.1|2)\r\n").unwrap();

        let (method, uri, _version) = match request_line_re.captures(&value) {
            Some(re) => (
                re.get(1).unwrap().as_str(),
                re.get(2).unwrap().as_str(),
                re.get(3).unwrap().as_str()
            ),
            // TODO: This below should end up with the server returning a HTTP 400 (Bad Request)
            None => return Err("Couldn't compile".to_string())
        };

        let mut headers: Headers = HashMap::new();

        for line in value.lines().skip(1).into_iter() {
            if line == "\r\n" {
                // Hit body or end.
                break;
            }
            
            let (name, value) = match line.split_once(": ") {
                Some(s) => s,
                None => continue
            };

            headers.insert(name.to_string(), value.to_string());
        }

        // TODO: Read Content-Length from the header here and then continue reading.

        Ok(Request {
            _method: method.to_string(),
            request_uri: uri.to_string(),
            _headers: headers
        })
    }
}