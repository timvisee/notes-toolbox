use doc::elements::Elements;

/// Element formatter interface.
/// Each element has it's own formatter for each document type.
pub trait ElementFormatter {
    /// Format the given element.
    ///
    /// `element` is the element that should be formatted.
    ///
    /// A vector with bytes is returned as formatted element.
    fn format(&self, element: &Elements) -> Vec<u8>;
}