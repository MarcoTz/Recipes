use super::errors::Error;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextElement {
    Plain(String),
    Link { label: String, target: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextBlock {
    pub elements: Vec<TextElement>,
}

impl fmt::Display for TextElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextElement::Plain(s) => f.write_str(s),
            TextElement::Link { label, target } => write!(f, "[{label}]({target})"),
        }
    }
}

impl fmt::Display for TextBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .elements
                .iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

impl From<&str> for TextElement {
    fn from(s: &str) -> TextElement {
        TextElement::Plain(s.to_owned())
    }
}

impl From<Vec<TextElement>> for TextBlock {
    fn from(elems: Vec<TextElement>) -> TextBlock {
        TextBlock { elements: elems }
    }
}

impl FromStr for TextBlock {
    type Err = Error;
    fn from_str(s: &str) -> Result<TextBlock, Self::Err> {
        let (non_link, link_start) = s.split_once('[').unwrap_or((s, ""));
        if link_start.is_empty() {
            return Ok(vec![non_link.into()].into());
        }

        let mut elements: Vec<TextElement> = vec![non_link.into()];
        let (link_label, rest) = link_start
            .split_once(']')
            .ok_or(Error::BracketMismatch("[".to_owned() + link_start))?;
        if !rest.starts_with('(') {
            return Err(Error::MissingLink(rest.to_owned()));
        }

        let (link_target, rest) = rest
            .split_once(')')
            .ok_or(Error::BracketMismatch(rest.into()))?;
        let mut target = link_target.to_owned();
        target.remove(0);
        elements.push(TextElement::Link {
            label: link_label.to_owned(),
            target,
        });

        let rest_blocks = rest.parse::<TextBlock>()?;
        elements.extend(rest_blocks.elements);
        elements.retain(|elem| {
            if let TextElement::Plain(st) = elem {
                !st.is_empty()
            } else {
                true
            }
        });

        Ok(elements.into())
    }
}

#[cfg(test)]
mod text_tests {
    use super::{TextBlock, TextElement};

    #[test]
    fn parse_non_link() {
        let result: TextBlock = "Text with no links".parse().unwrap();
        let expected = TextBlock {
            elements: vec![TextElement::Plain("Text with no links".to_owned())],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_link() {
        let result: TextBlock = "Text with [link](index.html)".parse().unwrap();
        let expected = TextBlock {
            elements: vec![
                TextElement::Plain("Text with ".to_owned()),
                TextElement::Link {
                    label: "link".to_owned(),
                    target: "index.html".to_owned(),
                },
            ],
        };
        assert_eq!(result, expected)
    }
}
