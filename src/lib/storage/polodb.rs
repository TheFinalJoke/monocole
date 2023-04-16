use polodb_core::Database;
use std::path::Path;
use std::fs::File;

use super::storage_trait::{self, StorageError};
pub struct PoloDb {
    pub db_path: String,
}


impl storage_trait::DbConnection<Database> for PoloDb {
    fn connect(&self) -> storage_trait::StorageResult<Database> {
        let db_conn = Database::open_file(&self.db_path);
        match db_conn {
            Ok(con) => Ok(con),
            Err(error) => Err(storage_trait::StorageError::PoloDbFileError(
                error.to_string(),
            )),
        }
    }
}

impl storage_trait::StoreDriver<Database> for PoloDb {
    fn insert(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }
    fn bulk_insert(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }

    fn create_table(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }

    fn create_db(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        let connection = conn.connect().unwrap();
        match connection.create_collection("monocole") {
            Err(e) => Err(storage_trait::StorageError::PoloDbError(e)),
            Ok(_) => Ok(())
        }
    }

    fn drop(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }

    fn del(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }

    fn select(
        &self,
        conn: impl storage_trait::DbConnection<Database>,
    ) -> storage_trait::StoreDriverResult {
        Ok(())
    }
}
impl PoloDb {
    fn check_existing_file(&self) -> bool {
        let path = Path::new(&self.db_path);
        path.exists()
    }

    pub fn check_file(&self) -> Result<(), storage_trait::StorageError> {
        if !self.check_existing_file(){
            let file = File::create(&self.db_path);
            return match file {
                Ok(_f) => Ok(()),
                Err(_) => Err(storage_trait::StorageError::PoloDbFileError(String::from("There was an error creating the polodb file")))
            };
        }
        Ok(())
    }
}