use crate::storage::storage_trait::{Storage, StoreDriver};

struct StorageDriver<D: StoreDriver> {
    driver: D,
}
