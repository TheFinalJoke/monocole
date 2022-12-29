// First Get the options that we need for cassandra
// Make sure we can get a connection with a password or Nah
// - TODO: Add Password and Username -> Add support for ENV
// Add Ability to have different Replications
// Network Topologies -> Allow Different Datacenters
// load_balancing
// Default Timeout
use crate::rpc_types::{config_types, hardware_types};
use scylla::frame::response::result::Row;
use scylla::transport::errors::NewSessionError;
use scylla::{Session, SessionBuilder};
use std::time::Duration;
use thiserror::Error;
use std::rc::Rc;

pub trait DataType {
    fn create_datatype(self) -> Box<Self>;
}
pub enum QueryStatus {
    COMPLETED=0,
    ADDED=1,
    FAILED=2,
    SUCCESS=3,
}
pub async fn connect(config_rules: Rc<config_types::ControllerConfigRules>,) -> Result<Session, NewSessionError> {
    let session = SessionBuilder::new()
        .known_nodes(&config_rules.db_server.as_ref().unwrap().host_ip)
        .connection_timeout(Duration::from_secs(5))
        .build()
        .await?;
    Ok(session)
}
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Connection was Interrupt")]
    ConnectionInterrupt(String),
    #[error("Invaild Query")]
    QueryInvalid(String),
    #[error("Could not establish connection")]
    ConnectionInvalid,
}
#[derive(Debug)]
pub struct Cql {
    pub keyspace: String,
    pub config_rules: Rc<config_types::ControllerConfigRules>,
    pub session: Session
}
impl Cql {
    pub async fn keyspaces(&self) -> Option<Vec<Row>>{
        match self.session.query("DESC KEYSPACES;", &[]).await {
            Ok(res) => res.rows,
            Err(_) => None
        }

    } 
    pub async fn build_keyspace(&self) -> Result<(), NewSessionError> {
        let options = self.config_rules.as_ref().cassandra_options.as_ref();
        self.session.query(
            format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': '{}', 'replication_factor': '{}'}}", 
            self.keyspace.as_str(),
            options.unwrap().replication().as_str_name(), options.unwrap().replication_factor.unwrap_or(1)),
            &[]).await?;
        Ok(())
    }

pub async fn develop_datatype<T,E>(&self, datatype: T) -> Result<(), E> {
        Ok(())
    }
}