extern crate iron;

use iron::BeforeMiddleware;

pub struct Fence {}

impl Fence {
    pub fn new() -> Fence {
        Fence {}
    }
}

impl BeforeMiddleware for Fence {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
