///! Middleware to record the server's response time.
extern crate iron;
extern crate time;

use std::io::net::ip::Ipv4Addr;
use iron::{Iron, Chain, Request, Response,
           Middleware, Server, Status,
           Continue, Unwind, FromFn};

use time::precise_time_ns;

#[deriving(Clone)]
struct ResponseTime {
    entry_time: u64
}

impl ResponseTime { fn new() -> ResponseTime { ResponseTime { entry_time: 0u64 } } }

impl Middleware for ResponseTime {
    fn enter(&mut self, _req: &mut Request, _res: &mut Response) -> Status {
        self.entry_time = precise_time_ns();
        Continue
    }

    fn exit(&mut self, _req: &mut Request, _res: &mut Response) -> Status {
        let delta = precise_time_ns() - self.entry_time;
        println!("Request took: {} ms", (delta as f64) / 100000.0);
        Continue
    }
}

fn no_op(_req: &mut Request, _: &mut Response) -> Status { Unwind }

fn main() {
    let mut server: Server = Iron::new();

    // This adds the ResponseTime middleware so that
    // all requests and responses are passed through it.
    server.chain.link(ResponseTime::new());
    server.chain.link(FromFn::new(no_op));

    // Start the server on localhost:3000
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

