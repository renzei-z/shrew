# TODOs and thoughts

- [ ] Multi-Threading?
    - We have an error (literally) when the client cuts off the connection (e.g. pressing X in a browser) which
        crashes the whole server with `Os { code: 10053, kind: ConnectionAborted }` :(
- [ ] Routing
    - For now, we are just going to directly match routes (/ -> /, /id/3 -> /id/3), but this is not going to be permanent,
        since we would like to have dynamic routing similar to ExpressJS.
    - How are we going to match routes like this though?
    - Do we just have some patterns and a pattern matcher function and loop through all routes in order of registration?
        - Isn't this slow? We'd like to just be able to look up the requested route like a `HashMap` but how do we do that when there's
            more than one way to represent a route? (e.g. /id/3 could be `/id/#` (where # is an undecided character for an integer) or `/id/3` etc.)
    - What happens if the user registers two identical routes?
- [ ] Responses
    - How do we make a general `res.send()` function that can accept most important data types?
    - Do we use an `&[u8]` or something?
    - Encoding?
    - How do we chunk the response body?
