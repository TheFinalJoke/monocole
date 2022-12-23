pub mod traits {
    use mockall::automock;

    #[automock]
    pub trait Retrieval<T: 'static> {
        fn retreieve<I: 'static>(self) -> Option<T>;
    }
}