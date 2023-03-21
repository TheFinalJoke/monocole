pub trait StoreDriver {
    fn insert<E>() -> Result<(), E>;

    fn bulk_insert<E>() -> Result<(), E>;

    fn create_table<E>() -> Result<(), E>;

    fn create_db<E>() -> Result<(), E>;

    fn drop<E>() -> Result<(), E>;

    fn del<E>() -> Result<(), E>;

    fn select<T, E>() -> Result<T, E>;
}

pub trait Storage {
    fn load<E>(driver: impl StoreDriver) -> Result<(), E>;

    fn connect<E>(&self) -> Result<(), E>;
}
