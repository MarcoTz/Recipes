use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseStep {
    Name,
    Ingredients,
    Steps,
    Notes,
    Tags,
    Done,
}

impl ParseStep {
    pub fn next(self) -> ParseStep {
        match self {
            ParseStep::Name => ParseStep::Ingredients,
            ParseStep::Ingredients => ParseStep::Steps,
            ParseStep::Steps => ParseStep::Notes,
            ParseStep::Notes => ParseStep::Tags,
            ParseStep::Tags => ParseStep::Done,
            ParseStep::Done => ParseStep::Done,
        }
    }
}

impl Default for ParseStep {
    fn default() -> ParseStep {
        ParseStep::Name
    }
}

impl fmt::Display for ParseStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseStep::Name => f.write_str("Name"),
            ParseStep::Ingredients => f.write_str("ingredients"),
            ParseStep::Steps => f.write_str("steps"),
            ParseStep::Notes => f.write_str("notes"),
            ParseStep::Tags => f.write_str("tags"),
            ParseStep::Done => f.write_str("done"),
        }
    }
}
