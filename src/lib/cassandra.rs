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
struct CQLQuery {
    keyspace: String,
    query: String,
}
impl CQLQuery {
    async fn connect(&self, hosts: &config_types::HostIp) -> Result<Session, NewSessionError> {
        let session = SessionBuilder::new()
            .known_nodes(&hosts.host_ip)
            .connection_timeout(Duration::from_secs(5))
            .build()
            .await?;
        Ok(session)
    }

    async fn build_keyspace(&self, config: config_types::ControllerConfigRules) -> Result<(), NewSessionError> {
        let hosts = config.db_server.unwrap_or_default();
        let session = self.connect(&hosts)
            .await?;
        session.query(
            ("CREATE KEYSPACE IF NOT EXISTS monocole WITH replication {'class': {}, 'replication_factor' {}", &config.cassandra_options.), 
            &[])?;
        Ok(())
    }
}