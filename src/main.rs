extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

mod controllers;

fn main() {
    let mut router = Router::new();

    router.get("/", controllers::friendly_controller::friendly_greeting, "message");

    Iron::new(router).http("localhost:3000").unwrap();
}
