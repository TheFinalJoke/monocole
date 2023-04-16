use thiserror::Error;
use polodb_core::DbErr;
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("PoloDb had an error, {0}")]
    PoloDbError(DbErr),

    #[error("PoloDb File Error {0}")]
    PoloDbFileError(String),

    #[error("Failed to open a connection to database")]
    PolodbConnectionFailure,
}
pub type StorageResult<T> = Result<T, StorageError>;
pub type StoreDriverResult = Result<(), StorageError>;


pub trait DbConnection<T> {
    fn connect(&self) -> StorageResult<T>;
}

pub trait StoreDriver<T> {
    fn insert(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn bulk_insert(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn create_table(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn create_db(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn drop(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn del(&self, conn: impl DbConnection<T>) -> StoreDriverResult;

    fn select(&self, conn: impl DbConnection<T>) -> StoreDriverResult;
}

pub trait Storage {
    fn load<T, E>(driver: impl StoreDriver<T>) -> Result<(), E>;

    fn connect<E>(&self) -> Result<(), E>;
}
