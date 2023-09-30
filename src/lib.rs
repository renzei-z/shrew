pub mod server;
pub mod request;
pub mod response;

pub mod prelude {
    pub use crate::{ 
        response::Response, 
        request::Request, 
        server::{ Server, ServerResult, RouteResult }
    };
}