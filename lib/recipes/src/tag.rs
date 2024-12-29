use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

impl Tag {
    pub fn get_url(&self, base: &str) -> String {
        format!("{base}/{}.html", self.0.replace(" ", ""))
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}
