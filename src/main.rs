extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

mod controllers;

fn main() {
    let mut router = Router::new();

    fn friendly_greeting(_: &mut Request) -> IronResult<Response> {
        return Ok(Response::with((status::Ok, "Hello World!")));
    }

    router.get("/", friendly_greeting, "message");

    Iron::new(router).http("localhost:3000").unwrap();
}
