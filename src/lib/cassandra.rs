// First Get the options that we need for cassandra
// Make sure we can get a connection with a password or Nah
// - TODO: Add Password and Username -> Add support for ENV
// Add Ability to have different Replications
// Network Topologies -> Allow Different Datacenters
// load_balancing
// Default Timeout
use crate::rpc_types::config_types;
use scylla::transport::errors::NewSessionError;
use scylla::{Session, SessionBuilder};
use std::time::Duration;
use thiserror::Error;
use std::rc::Rc;

pub trait DataType<T: 'static, E: 'static> {
    fn create_datatype<I: 'static>(self) -> Result<T, E>;
}
pub enum QueryStatus {
    COMPLETED=0,
    ADDED=1,
    FAILED=2,
    SUCCESS=3,
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
#[derive(Debug, Clone)]
pub struct CQLQuery {
    pub keyspace: String,
    pub cassandra_options: config_types::CassandraOptions,
    pub db_server: Rc<Option<config_types::HostIp>>,
}
impl CQLQuery {
    pub async fn connect(&self) -> Result<Session, NewSessionError> {
        let session = SessionBuilder::new()
            .known_nodes(self.db_server.as_deref())
            .connection_timeout(Duration::from_secs(5))
            .build()
            .await?;
        Ok(session)
    }

    pub async fn build_keyspace(&self) -> Result<(), NewSessionError> {
        let session = self.connect()
            .await?;
        session.query(
            format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': '{}', 'replication_factor': '{}'}}", 
            self.keyspace.as_str(),
            self.cassandra_options.replication().as_str_name(), self.cassandra_options.replication_factor.unwrap_or(1)),
            &[]).await?;
        Ok(())
    }

    pub async fn develop_datatype<T,E>(&self, datatype: T) -> Result<(), E> {
        println!("Any datatype that impl the datatype struct");
        Ok(())

    }
}