use itertools::Itertools;
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
    // let re = Regex::new(r"[\n\r]|```(?:\w+\n)?[\s\S]*?```|.*?[!.?][ \t]*|.*?\z")?;
    let re =
        Regex::new(r"`[\n\r]|```(?:\w+\n)?[\s\S]*?```|.*?(?:(?:!|\?|\n)|(?:\.\s+))[ \t\s]*|.*?\z")?;
    output.push(
        re.find_iter(input)
            .map(|m| m.as_str().replace(['\n', '\r'], "<br />"))
            .map(|s| {
                if s.is_empty() || s == ("<br />") {
                    s
                } else if s.ends_with("<br />") {
                    let br = s.rmatches("<br />").join("");
                    let s = s.split("<br />").join("");
                    "<s>".to_owned() + &s + "</s>" + &br
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

    #[test]
    fn currencies_test() {
        let input = "It's a standing machine that dispenses coffee. You can pay for drinks using the balance in your student account by tapping your payment method against the card reader, and they'll be dispensed by the COFFEE NOZZLE below the panel, after you've put a cup under it. There are cups in the CUP TRAY on the side of the machine. You can make a purchase by sending `.use COFFEE MACHINE ` followed by one of the options listed below.

The following options are available:
- `COFFEE`: $1.00
- `COFFEE WITH SUGAR`: $1.00
- `COFFEE WITH MILK`: $1.50
- `COFFEE WITH CREAMER`: $1.50
- `COFFEE WITH MILK AND SUGAR`: $1.50
- `COFFEE WITH CREAMER AND SUGAR`: $1.50";
        let expected = "<desc><s>It's a standing machine that dispenses coffee.</s> <s>You can pay for drinks using the balance in your student account by tapping your payment method against the card reader, and they'll be dispensed by the COFFEE NOZZLE below the panel, after you've put a cup under it.</s> <s>There are cups in the CUP TRAY on the side of the machine.</s> <s>You can make a purchase by sending `.use COFFEE MACHINE ` followed by one of the options listed below.</s><br /><br /><s>The following options are available:</s><br /><s>- `COFFEE`: $1.00</s><br /><s>- `COFFEE WITH SUGAR`: $1.00</s><br /><s>- `COFFEE WITH MILK`: $1.50</s><br /><s>- `COFFEE WITH CREAMER`: $1.50</s><br /><s>- `COFFEE WITH MILK AND SUGAR`: $1.50</s><br /><s>- `COFFEE WITH CREAMER AND SUGAR`: $1.50</s></desc>";

        let result = str_to_description(input);

        assert_eq!(expected, result.unwrap().as_str());
    }

    #[test]
    fn codeblock_test() {
        let input = "It's a large, white refrigerator with a FREEZER compartment on top. When you open up the fridge, you see SHELF 1, SHELF 2, and SHELF 3. At the bottom are two crispers: CRISPER 1 and CRISPER 2. The inside of the door has some TRAYS.\n\nThere's a laminated piece of paper hung on the door of the fridge. It reads:\n```\nREFRIGERATOR RULES:\n1. Don't overfill it!\n2. Follow the labels when putting things inside\n3. Sharing is caring :)\n4. Let the group chat know when you've made extra\n\n- Adelaide Castelo\n```";
        let expected = "<desc><s>It's a large, white refrigerator with a FREEZER compartment on top.</s> <s>When you open up the fridge, you see SHELF 1, SHELF 2, and SHELF 3.</s> <s>At the bottom are two crispers: CRISPER 1 and CRISPER 2.</s> <s>The inside of the door has some TRAYS.</s><br /><br /><s>There's a laminated piece of paper hung on the door of the fridge.</s> <s>It reads:</s><br /><s>```<br />REFRIGERATOR RULES:<br />1. Don't overfill it!<br />2. Follow the labels when putting things inside<br />3. Sharing is caring :)<br />4. Let the group chat know when you've made extra<br /><br />- Adelaide Castelo<br />```</s></desc>";

        let result = str_to_description(input);

        assert_eq!(expected, result.unwrap());
    }
}
