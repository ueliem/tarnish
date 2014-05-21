use request;
use response;

#[deriving(Clone)]//Automatically implements everything for the Clone trait.
pub struct RustRouter {
    pub num: uint,
}

trait Router: Send + Clone {
    fn add_handler(verb: &str, handler: &fn(request::Request,response::Response));
}

impl Router for RustRouter {
    fn add_handler(verb: &str, handler: &fn(request::Request,response::Response)) {
            match verb {
                "get"   => fail!(""),
                "put"   => fail!(""),
                _       => fail!("Not a verb"),
            }
    }
}
