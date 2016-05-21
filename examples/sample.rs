extern crate iron;
extern crate fence;

use iron::prelude::*;
use iron::status;
use fence::Fence;

fn main() {
    let fence = Fence::new();

    let mut chain = Chain::new(hello_handler);

    chain.link_before(fence);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn hello_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}
