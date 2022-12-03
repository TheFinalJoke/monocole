//use lib::cassandra;
extern crate log;
use lib::logging::MyLogger;
use clap::{Arg, Command};
#[tokio::main]
async fn main() {
    // Create Logger
    MyLogger::init().unwrap();
    let app = Command::new("monocole-agent")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .help("Path to configuration")
            .default_value("/etc/monocole/monocole.yaml"));
    
    let matches = app.get_matches();
    let path = matches.get_one::<String>("path");
    let configure = lib::get_agent_configuration(
        path.expect("Invalid path").to_owned()
    );
    dbg!(configure);
}