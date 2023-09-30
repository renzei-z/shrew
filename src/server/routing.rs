use super::*;

use std::collections::HashMap;

pub type RouteResult<'r> = std::io::Result<Response<'r>>;

// HashMap that can be returned is a map of all parameters
fn match_uri(uri: &str, route: &str) -> (bool, Option<HashMap<String, String>>) {
    if uri == route {
        return (true, None);
    }

    // / -> []
    // /%id -> ["%id"]
    let split_route: Vec<&str> = route.split("/").collect();
    let split_uri: Vec<&str> = uri.split("/").collect();

    if split_route.len() != split_uri.len() {
        return (false, None);
    }

    let mut params = HashMap::<String, String>::new();
    let mut matches = true;

    for (subroute, suburi) in split_route.iter().zip(split_uri.iter()) {
        if subroute.starts_with("%") {
            let param_name = subroute.split_at(1).1;
            params.insert(param_name.to_string(), suburi.to_string());
            continue;
        }

        if subroute == suburi { continue; }
        matches = false;
        break;
    }
    
    match matches {
        false => (false, None),
        true => (true, Some(params))
    }
}

pub fn default_fallback_route(_req: Request, res: Response) -> RouteResult<'_> {
    res.set_status(404).send("404 Not Found")
}

impl Server {
    pub fn get(&mut self, route: &str, response: RoutingFunction) -> ServerResult {
        if self.routes.iter().any(|r| r.0 == route.to_string()) {
            return Err(ServerError::DuplicateRoute(route.to_string()));
        }
        self.routes.push((route.to_string(), response));
        Ok(())
    }

    pub(crate) fn route<'r>(&'r mut self, mut req: Request, res: Response<'r>) -> RouteResult<'r> {
        // TODO: Caching?
        for route in self.routes.iter() {
            if let (true, p) = match_uri(&req.request_uri, &route.0) {
                if let Some(params) = p {
                    req.params.extend(params);
                }

                return (route.1)(req, res);
            }
        }
    
        // If we haven't returned, we haven't found the route.

        (self.fallback_route)(req, res)
    }
}