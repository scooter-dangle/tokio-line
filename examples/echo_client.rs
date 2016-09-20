extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_line as line;
extern crate tokio_service as service;
extern crate env_logger;

use tokio::reactor::Core;
use service::Service;

pub fn main() {
    env_logger::init().unwrap();

    let mut core = Core::new().unwrap();

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    let client = line::client::connect(core.handle(), &addr);
    let client = core.run(client).unwrap();

    // The connect call returns us a ClientHandle that allows us to use the 'Service' as a function
    // - one that returns a future that we can 'await' on.
    let resp = client.call("Hello".to_string());
    println!("RESPONSE: {:?}", core.run(resp));
}
