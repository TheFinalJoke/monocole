//use lib::cassandra;
#[macro_use]
extern crate log;
use lib::logging::MyLogger;
use lib::cassandra;

#[tokio::main]
async fn main() {
    // Create Logger
    MyLogger::init().unwrap();
    info!("hello from contoller");
    cassandra::run().await;
    trace!("After Cassandra");
}