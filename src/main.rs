extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;
use controllers::friendly_controller;

mod controllers;

fn main() {
    let mut router = Router::new();

    router.get("/", friendly_controller::friendly_greeting, "message");

    Iron::new(router).http("localhost:3000").unwrap();
}
