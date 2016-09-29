use iron::status;
use iron::prelude::*;

pub fn friendly_greeting(_: &mut Request) -> IronResult<Response> {
    return Ok(Response::with((status::Ok, "Hello World!")));
}