use crate::storage::storage_trait;

struct PoloDb {
    pub db_path: String,
}

impl storage_trait::StoreDriver for PoloDb {
    fn insert<E>() -> Result<(), E> {
        todo!()
    }

    fn bulk_insert<E>() -> Result<(), E> {
        todo!()
    }

    fn create_table<E>() -> Result<(), E> {
        todo!()
    }

    fn create_db<E>() -> Result<(), E> {
        todo!()
    }

    fn drop<E>() -> Result<(), E> {
        todo!()
    }

    fn del<E>() -> Result<(), E> {
        todo!()
    }

    fn select<T, E>() -> Result<T, E> {
        todo!()
    }
}
