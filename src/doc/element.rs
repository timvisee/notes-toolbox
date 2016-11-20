pub trait Element {
    /// Get the name of the type of element.
    fn get_type_name() -> &'static str;
}