pub trait ToString {
    fn to_string_ref(&self) -> &'static str;
}

pub trait Part<T: 'static> {
    fn to_string(&self) -> String;

    fn get_field_names(&self) -> Vec<&'static str>;
}

// Find a way to have a trait of parts and that we can interact with
