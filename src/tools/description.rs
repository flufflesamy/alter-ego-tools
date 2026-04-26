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
    let re = Regex::new(r"[\n\r]|```(?:\w+\n)?[\s\S]*?```|.*?[!.?][ \t]*|.*?\z")?;
    output.push(
        re.find_iter(input)
            .map(|m| m.as_str().replace(['\n', '\r'], "<br />"))
            .map(|s| {
                if s.is_empty() {
                    "".to_owned()
                } else if s == ("<br />") || s.contains("```") {
                    s
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
        let expected = "<desc><s>This is a ROOM.</s> <s>It has two PEOPLE in it!</s><s>There is a BUNNY there too?</s><br /></desc>";

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

    #[test]
    fn bullet_points_test() {
        let input = "Keep our school looking beautiful!

- All students are responsible for cleaning up after themselves.
- Dishes are to be washed in the kitchen after use and returned to their proper locations.
- All trays must be cleaned and returned to the dining hall after use.
- Please wipe your tables before leaving.

Thank you!";
        let result = str_to_description(input);
        let expected = "<desc><s>Keep our school looking beautiful!</s><br /><br /><s>- All students are responsible for cleaning up after themselves.</s><br /><s>- Dishes are to be washed in the kitchen after use and returned to their proper locations.</s><br /><s>- All trays must be cleaned and returned to the dining hall after use.</s><br /><s>- Please wipe your tables before leaving.</s><br /><br /><s>Thank you!</s></desc>";

        assert_eq!(expected, result.unwrap().as_str());
    }
}

