// SPDX-FileCopyrightText: 2026 Amy Poon <amy@amypoon.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Result, anyhow};
use itertools::Itertools;
use regex::{Captures, Regex};

#[derive(Debug, Default)]
pub struct CommandSet {
    outcomes: Vec<String>,
    solved_commands: Vec<String>,
    unsolved_commands: Vec<String>,
    capture_string: String,
}

#[allow(dead_code)]
impl CommandSet {
    pub fn builder() -> CommandSetBuilder {
        CommandSetBuilder::default()
    }

    pub fn generate(&self) -> Result<String> {
        // Generate regex from capture string
        let regex = Regex::new(&self.capture_string)?;

        // Get captures from regex and zip with outcomes
        let outcomes_captures: Vec<(&str, Captures<'_>)> = self
            .outcomes
            .iter()
            .map(|o| -> Result<(&str, Captures<'_>)> {
                Ok((
                    o,
                    regex
                        .captures(o)
                        .ok_or_else(|| anyhow!("There are no matches"))?,
                ))
            })
            .try_collect()?;

        // Formats the outcomes and solved commands into command set string
        let res = outcomes_captures
            .iter()
            .map(|(o, c)| {
                let solved = Self::replace_commands(&self.solved_commands, c);
                let unsolved = Self::replace_commands(&self.unsolved_commands, c);

                if solved.is_empty() && unsolved.is_empty() {
                    format!("[{o}]")
                } else if unsolved.is_empty() {
                    format!("[{o}: {solved}]")
                } else {
                    format!("[{o}: {solved} / {unsolved}]")
                }
            })
            .join(", ");

        Ok(res)
    }

    fn replace_commands(commands: &[String], captures: &Captures<'_>) -> String {
        commands
            .iter()
            .map(|s| {
                let mut temp = s.clone();

                temp = temp.replace("/", "\\");
                temp = temp.replace(":", "@");
                if temp.starts_with("say")
                    || temp.starts_with("fixture")
                    || temp.starts_with("puzzle")
                    || temp.starts_with("setvoice")
                {
                    temp = temp.replace(",", "，");
                }

                for (i, m) in captures.iter().enumerate() {
                    // Skip first
                    if i == 0 {
                        continue;
                    };

                    // Replaces e.g. $1 into match 1
                    if let Some(m) = m {
                        temp = temp.replace(format!("${i}").as_str(), m.as_str());
                    }
                }
                temp
            })
            .join(", ")
    }
}

#[derive(Debug, Default)]
pub struct CommandSetBuilder {
    outcomes: Vec<String>,
    solved_commands: Vec<String>,
    unsolved_commands: Vec<String>,
    capture_string: String,
}

#[allow(dead_code)]
impl CommandSetBuilder {
    pub fn outcome(&mut self, outcome: String) -> &mut Self {
        self.outcomes.push(outcome);
        self
    }

    pub fn outcomes(&mut self, outcomes: Vec<String>) -> &mut Self {
        self.outcomes.extend(outcomes);
        self
    }

    pub fn solved_command(&mut self, solved_command: String) -> &mut Self {
        self.solved_commands.push(solved_command);
        self
    }

    pub fn solved_commands(&mut self, solved_commands: Vec<String>) -> &mut Self {
        self.solved_commands.extend(solved_commands);
        self
    }

    pub fn unsolved_command(&mut self, unsolved_command: String) -> &mut Self {
        self.unsolved_commands.push(unsolved_command);
        self
    }

    pub fn unsolved_commands(&mut self, unsolved_commands: Vec<String>) -> &mut Self {
        self.unsolved_commands.extend(unsolved_commands);
        self
    }

    pub fn capture_string(&mut self, capture_string: String) -> &mut Self {
        self.capture_string = capture_string;
        self
    }

    pub fn build(&self) -> Result<CommandSet> {
        if self.outcomes.is_empty() {
            Err(anyhow!("At least one outcome must be provided"))
        } else if self.capture_string.is_empty() {
            Err(anyhow!("Capture string must be provided"))
        } else if self.solved_commands.is_empty() && self.unsolved_commands.is_empty() {
            Err(anyhow!(
                "At least one solved command or unsolved command must be provided"
            ))
        } else {
            Ok(CommandSet {
                outcomes: self.outcomes.clone(),
                solved_commands: self.solved_commands.clone(),
                unsolved_commands: self.unsolved_commands.clone(),
                capture_string: self.capture_string.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_ok() {
        let mut builder = CommandSet::builder();
        builder.outcome("PINK SPIRAL NOTEBOOK, pink spiral notebook".to_owned());
        builder.capture_string(".*? (NOTEBOOK), (.*?) (.*?)".to_owned());
        builder.solved_command(
            "is $1 (notebook color=$2+notebook type=$3) in BAGGING AREA at school-store".to_owned(),
        );
        builder.solved_command("say Ai {Compliment}: That is, an excellent choice.".to_owned());
        builder.solved_command("setpronouns Ai it/it/its/its/itself/false".to_owned());
        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_builder_err() {
        let mut builder = CommandSet::builder();
        builder.outcome("PINK SPIRAL NOTEBOOK, pink spiral notebook".to_owned());
        builder.capture_string(".*? (NOTEBOOK), (.*?) (.*?)".to_owned());
        assert!(builder.build().is_err()); // No solved or unsolved commands

        let mut builder = CommandSet::builder();
        builder.outcome("PINK SPIRAL NOTEBOOK, pink spiral notebook".to_owned());
        builder.solved_command(
            "is $1 (notebook color=$2+notebook type=$3) in BAGGING AREA at school-store, say Ai \
             {Compliment}: That is, an excellent choice."
                .to_owned(),
        );
        assert!(builder.build().is_err()); // No capture string

        let mut builder = CommandSet::builder();
        builder.capture_string(".*? (NOTEBOOK), (.*?) (.*?)".to_owned());
        builder.solved_command(
            "is $1 (notebook color=$2+notebook type=$3) in BAGGING AREA at school-store, say Ai \
             {Compliment}: That is, an excellent choice."
                .to_owned(),
        );
        assert!(builder.build().is_err()); // No outcomes
    }

    #[test]
    fn test_command_set_generate1() {
        let mut builder = CommandSet::builder();
        builder.outcome("PINK SPIRAL NOTEBOOK, pink spiral notebook".to_owned());
        builder.outcome("BLACK COMPOSITION NOTEBOOK, black composition notebook".to_owned());
        builder.capture_string(r".*? (NOTEBOOK), (\w+) (\w+)".to_owned());
        builder.solved_command(
            "is $1 (notebook color=$2+notebook type=$3) in BAGGING AREA at school-store".to_owned(),
        );
        builder.solved_command("say Ai {Compliment}: That is, an excellent choice.".to_owned());
        builder.solved_command("setpronouns Ai it/it/its/its/itself/false".to_owned());

        let command_set = builder.build().unwrap();
        let output = command_set.generate().unwrap();

        assert_eq!(
            output,
            "[PINK SPIRAL NOTEBOOK, pink spiral notebook: is NOTEBOOK (notebook \
             color=pink+notebook type=spiral) in BAGGING AREA at school-store, say Ai \
             {Compliment}@ That is， an excellent choice., setpronouns Ai \
             it\\it\\its\\its\\itself\\false], [BLACK COMPOSITION NOTEBOOK, black composition \
             notebook: is NOTEBOOK (notebook color=black+notebook type=composition) in BAGGING \
             AREA at school-store, say Ai {Compliment}@ That is， an excellent choice., \
             setpronouns Ai it\\it\\its\\its\\itself\\false]"
        );
    }

    #[test]
    fn test_command_set_generate2() {
        let mut builder = CommandSet::builder();
        builder.outcome("Item: TABLET 0, Item: TABLET 1".to_owned());
        builder.capture_string(r"Item: (.*?), Item: (.*) (\d+)".to_owned());
        builder.solved_command("solve SWITCH dorm-$3, unlock dorm-$3 DOOR".to_owned());
        builder.unsolved_command("unsolve SWITCH dorm-$3, lock dorm-$3 DOOR".to_owned());

        let command_set = builder.build().unwrap();
        let output = command_set.generate().unwrap();

        assert_eq!(
            output,
            "[Item: TABLET 0, Item: TABLET 1: solve SWITCH dorm-1, unlock dorm-1 DOOR / unsolve \
             SWITCH dorm-1, lock dorm-1 DOOR]"
        );
    }
}
