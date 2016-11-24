use std::hash::Hash;

/// Elements enum.
/// This enum contains all supported document elements and their data.
#[derive(Hash, Eq, PartialEq)]
pub enum Elements {
    /// Regular text element
    Text {
        text: Vec<u8>
    },

    /// Bold text element
    BoldText {
        text: Vec<u8>
    },

    /// Header element
    ///
    /// Where `level` is the depth of the header, starting at `1` going up.
    Header {
        level: u8,
        text: Vec<u8>
    }
}
