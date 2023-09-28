#[derive(Default)]
pub struct Request {
    // TODO: Turn this into an enum
    method: String,
    pub request_uri: String
}

impl Request {
    // TODO: Use a Result with a custom error type.
    pub fn from_string(string: String) -> Option<Request> {
        let mut request = Request::default();

        // TODO: For now, we are *assuming* that the HTTP request is
        // valid; i.e. the first line contains the method and route.
        // This is obviously insecure, and will be fixed soon after
        // simple routing and header reading is implemented.

        // The naming for variables in this section comes from RFC 2616. (HTTP)
        let request_line = string.split_at(string.find("\r\n").unwrap()).0;

        // request_line.0 should contain 'Method SP Request-URI SP HTTP-Version'
        let split_request_line: Vec<_> = request_line.split(' ').collect();

        request.request_uri = split_request_line[1].to_string();

        Some(request)
    }
}