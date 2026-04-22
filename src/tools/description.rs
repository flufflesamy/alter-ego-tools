use regex::Regex;

/// This function converts a string slice to a String containing an Alter Ego Description.
///
/// # Errors
///
/// This function will return an error if Regex fails.
pub fn str_to_description(input: &str) -> anyhow::Result<String> {
    // Add first desc tag
    let mut output: Vec<String> = vec!["<desc>".to_owned()];

    // Split into sentences with <s> tags
    // let re = Regex::new(r".*?[!.?]\s*")?;
    let re = Regex::new(r".*?[!.?]\s*|.*?\z")?;
    output.push(
        re.find_iter(input)
            .map(|m| m.as_str().replace("\n", "<br />"))
            .map(|s| {
                if s.is_empty() {
                    "".to_owned()
                } else if s.ends_with(" ") {
                    "<s>".to_owned() + s.trim_end() + "</s> "
                } else {
                    "<s>".to_owned() + s.as_str() + "</s>"
                }
            })
            .collect(),
    );

    // Add last desc tag
    output.push("</desc>".to_owned());

    Ok(output.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_description_test() {
        let input = "This is a ROOM. It has two PEOPLE in it!There is a BUNNY there too?\n";
        let result = str_to_description(input);
        let expected = "<desc><s>This is a ROOM.</s> <s>It has two PEOPLE in it!</s><s>There is a BUNNY there too?<br /></s></desc>";

        assert_eq!(expected, result.unwrap().as_str());
    }

    #[test]
    fn empty_description_test() {
        let input = "";
        let result = str_to_description(input);
        let expected = "<desc></desc>";

        assert_eq!(expected, result.unwrap().as_str());
    }

    #[test]
    fn no_periods_test() {
        let input = "This is a ROOM, there are three RABBITS here";
        let result = str_to_description(input);
        let expected = "<desc><s>This is a ROOM, there are three RABBITS here</s></desc>";

        assert_eq!(expected, result.unwrap().as_str());
    }
}
