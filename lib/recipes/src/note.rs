use super::TextBlock;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Note(pub TextBlock);

impl From<TextBlock> for Note {
    fn from(block: TextBlock) -> Note {
        Note(block)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
