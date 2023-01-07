//use lib::cassandra;
extern crate log;
use clap::{Arg, Command};
use lib::logging::MyLogger;
use lib::retrieval;
use lib::cassandra;
use scylla::IntoTypedRows;
use std::rc::Rc;
use std::error::Error;

static KEYSPACE: &str = "monocole";

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{
    // Create Logger
    MyLogger::init().unwrap();
    let app = Command::new("monocole")
    .arg(
        Arg::new("path")
            .short('p')
            .long("path")
            .help("Path to configuration")
            .default_value("/etc/monocole/monocole.yaml"),
    )
    .arg(
        Arg::new("drop")
            .short('d')
            .long("drop")
            .help("Drop the Keyspace and all data after termination")
            .action(clap::ArgAction::SetTrue)
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
    // Check if there is a keyspaces created
    // Maybe turn this into a function
    if let Some(rows) = query.keyspaces().await {
        let mut keyspaces: Vec<bool> = Vec::new();
        for row in rows.into_typed::<(String, String, String)>() {
            if row.unwrap().0.as_str() != "monocole" {
                keyspaces.push(false);
            } else {
                keyspaces.push(true);
            }
        };
        if keyspaces.contains(&true) {
            log::info!("Keyspace monocole is already created");
        } else {
            query.build_keyspace().await?;
            query.develop_datatypes().await?;
            // Generate datatypes 
            // Generate Tables
            // if dev Generate rows for development
        }

    }
    if *matches.get_one::<bool>("drop").unwrap() {
        log::info!("Dropping Keyspaces");
        let session = lib::cassandra::connect(Rc::clone(&referenced_config));
        let query = cassandra::Cql{
            keyspace: KEYSPACE.to_string(),
            config_rules: Rc::clone(&referenced_config),
            session: session.await.expect("Failed to Connect"),
        };
        query.drop_keyspace().await?;
    }
    Ok(())
}
