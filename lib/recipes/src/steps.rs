use super::TextBlock;
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StepSection {
    pub header: String,
    pub steps: Vec<TextBlock>,
}
impl From<Vec<TextBlock>> for StepSection {
    fn from(steps: Vec<TextBlock>) -> StepSection {
        StepSection {
            header: "".to_owned(),
            steps,
        }
    }
}

impl fmt::Display for StepSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let step_strs: Vec<String> = self
            .steps
            .iter()
            .enumerate()
            .map(|(num, step)| format!("{}. {step}", num + 1))
            .collect();
        if !self.header.is_empty() {
            writeln!(f, "### {}", self.header)?;
            writeln!(f)?;
        }
        writeln!(f, "{}", step_strs.join("\n"))
    }
}
