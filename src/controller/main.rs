//use lib::cassandra;
extern crate log;
use clap::{Arg, Command};
use lib::logging::MyLogger;
use lib::retrieval;
use lib::cassandra;
use scylla::IntoTypedRows;
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
    let referenced_config = Rc::new(configure);
    let session = lib::cassandra::connect(Rc::clone(&referenced_config));
    let query = cassandra::Cql{
        keyspace: KEYSPACE.to_string(),
        config_rules: Rc::clone(&referenced_config),
        session: session.await.expect("Failed to Connect"),
    };
    //query.keyspaces().await.into_iter().for_each(|row| {
    //    println!(r#"{:?}"#, row[0].columns);
    //})

    if let Some(rows) = query.keyspaces().await {
        // Parse each row as a tuple containing single i32
        rows.into_typed::<(String,String, String)>().into_iter().for_each(|row| {
            if row.unwrap().0 == "monocole" {
            
            }
        });
    }
    
}
