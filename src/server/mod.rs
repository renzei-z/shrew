mod stream;

use super::{ request::Request, response::Response };

use std::net::{ TcpListener, TcpStream };
use std::collections::HashMap;
pub use self::routing::RouteResult;
pub use self::stream::Stream;

pub struct Server {
    routes: HashMap<String, Vec<(String, RoutingFunction)>>,
    fallback_route: RoutingFunction
}

#[derive(Debug)]
pub enum ServerError {
    DuplicateRoute(String)
}

type RoutingFunction = fn(Request, Response) -> RouteResult;
pub type ServerResult = Result<(), ServerError>;

mod routing;

impl Server {
    pub fn new() -> Self {
        Server {
            routes: HashMap::new(),
            fallback_route: routing::default_fallback_route
        }
    }

    fn accept_stream(&mut self, tcp_stream: &mut TcpStream) {
        let stream = Stream::from_stream(tcp_stream);
        let request = match stream.get_request() {
            Some(request) => request,
            None => return
        };

        // TODO: Fix the fact we have to make a second struct??
        let response = Response::new(Stream::from_stream(tcp_stream));
        self.route(request, response).unwrap();
    }

    pub fn listen(&mut self, address: &str, port: usize, on_listen: fn()) -> ServerResult {
        let listener = match TcpListener::bind(format!("{address}:{port}")) {
            Ok(l) => {
                on_listen();
                l
            },
            Err(_error) => {
                panic!("Handle binding error!!");
            }
        };

        for stream in listener.incoming() {
            self.accept_stream(&mut stream.unwrap());
        }

        Ok(())
    }

    /// Start the server listening on a port
    /// on localhost.
    /// 
    /// Alias for `Server::listen("[::1]", port, on_listen)`.
    pub fn localhost(&mut self, port: usize, on_listen: fn()) -> ServerResult {
        self.listen("[::1]", port, on_listen)
    }
}