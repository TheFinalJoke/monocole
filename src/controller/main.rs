//use lib::cassandra;
extern crate log;
use clap::{Arg, Command};
use lib::logging::MyLogger;
use lib::retrieval;
use lib::cassandra;
use std::rc::Rc;

static KEYSPACE: &str = "monocole";

#[tokio::main]
async fn main() {
    // Create Logger
    MyLogger::init().unwrap();
    let app = Command::new("monocole").arg(
        Arg::new("path")
            .short('p')
            .long("path")
            .help("Path to configuration")
            .default_value("/etc/monocole/monocole.yaml"),
    );

    let matches = app.get_matches();
    let path = matches.get_one::<String>("path");
    let configure = lib::get_controller_configuration(retrieval::config_lib::FileRetrieve {
        path: path.expect("Invalid path").to_owned(),
    });
    dbg!(&configure);
    let query = cassandra::CQLQuery{
        keyspace: KEYSPACE.to_string(),
        cassandra_options: configure.cassandra_options.unwrap(),
        db_server: configure.db_server
    };
    dbg!(configure.db_server);
    query.build_keyspace().await;

}
