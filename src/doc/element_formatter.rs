use doc::elements::Elements;

pub trait ElementFormatter {
    /// Format the given element.
    ///
    /// `element` is the element that should be formatted.
    ///
    /// A vector with bytes is returned as formatted element.
    fn format(&self, element: &Elements) -> Vec<u8>;
}