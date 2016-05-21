extern crate iron;

use iron::prelude::*;
use iron::AfterMiddleware;
use iron::status;

pub struct Fence {}

impl Fence {
    pub fn new() -> Fence {
        Fence {}
    }
}

impl AfterMiddleware for Fence {
    fn after(&self, req: &mut Request, _: Response) -> IronResult<Response> {
        println!("{:?}", req.remote_addr);
        Ok(Response::with((status::TooManyRequests)))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
