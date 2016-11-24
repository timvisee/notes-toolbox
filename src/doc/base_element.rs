// TODO: This should be removed when we're sure the element interface system isn't used anymore.

#[allow(dead_code)]

/// Element interface, may be used by element implementations
pub trait BaseElement {
    /// Get the name of the type of element.
    fn get_type_name() -> &'static str;
}