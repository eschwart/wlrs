mod cfg;
mod env;
mod err;
mod utils;

use std::net::TcpListener;

use cfg::*;
use err::*;
use utils::*;

fn main() -> Result<()> {
    let cfg = Config::new();
    let server = TcpListener::bind(env::SERVER_ADDR)?;
    let limiter = RateLimiter::new(cfg.rate_limit());

    println!("Listening @ http://{}\n", server.local_addr()?);

    // handle each stream individually
    for s in server.incoming().filter_map(std::io::Result::ok) {
        _ = handle_stream(s, limiter.clone());
    }
    unreachable!()
}
