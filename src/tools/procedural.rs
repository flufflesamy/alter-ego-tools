use anyhow::{Result, bail};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Stat {
    Strength,
    Perception,
    Dexterity,
    Speed,
    Stamina,
}

impl ToString for Stat {
    /// Converts the given Stat to a short string.
    ///
    /// # Examples
    /// ```rs
    /// let stat_string = Stat::Perception.to_string();
    /// let per = String::from("per");
    ///
    /// assert_eq!(per, stat_string);
    /// ```
    fn to_string(&self) -> String {
        match self {
            Stat::Strength => "str".into(),
            Stat::Perception => "per".into(),
            Stat::Dexterity => "dex".into(),
            Stat::Speed => "spd".into(),
            Stat::Stamina => "sta".into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PossibleFlag {
    Uppercase,
    Lowercase,
}

pub type Possibility = (Option<String>, Option<f64>);

#[derive(Debug, Clone, PartialEq)]
pub struct Procedural {
    name: Option<String>,
    chance: Option<f64>,
    stat: Option<Stat>,
    possibilities: Vec<Possibility>,
}

impl Procedural {
    pub fn builder() -> ProceduralBuilder {
        ProceduralBuilder::default()
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn chance(&self) -> &Option<f64> {
        &self.chance
    }

    pub fn stat(&self) -> &Option<Stat> {
        &self.stat
    }

    pub fn possibilities(&self) -> &Vec<Possibility> {
        &self.possibilities
    }

    /// Generates Alter Ego procedural string.
    ///
    /// Uses the fields of the Procedural struct to generate a string.
    ///
    /// # Examples
    /// ```rs
    /// let procedural = ProceduralBuilder::new().build();
    /// let generated = procedural.generate();
    /// let proc_string = String::from("<procedural></procedural>");
    ///
    /// assert_eq!(proc_string, generated);
    /// ```
    pub fn generate_procedural(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        // Push <procedural...>
        parts.push(self.generate_proc_tag());
        // Push possibilities
        parts.push(self.generate_poss_tags());
        // Push closing </procedural>
        parts.push("</procedural>".into());
        parts.join("")
    }

    /// Generates Alter Ego possible names string.
    ///
    /// Procedural must have a name and at least one possibility must be named, or function returns error.
    ///
    /// # Examples
    pub fn generate_possible_names(&self, flag: PossibleFlag) -> Result<String> {
        // If procedural doesn't have name, bail
        let name = self.get_some_name()?;

        // Turn possibilities into tags
        let possibilities_tags: Vec<String> = self
            .get_named_possibilities()?
            .into_iter()
            .map(|p| self.to_possible_name_tag(name, &p, flag))
            .collect();

        // Join tags into string
        Ok(self.join_tags(possibilities_tags))
    }

    /// Generates Alter Ego possible containing phrases string.
    ///
    /// Procedural must have a name and at least one possibility must be named, or function returns error.
    ///
    /// # Examples
    pub fn generate_possible_containing_phrases(
        &self,
        pattern: &str,
        flag: PossibleFlag,
    ) -> Result<String> {
        let name = self.get_some_name()?;

        let possibilities_tags: Vec<String> = self
            .get_named_possibilities()?
            .into_iter()
            .map(|p| self.to_possible_phrase_tag(name, &p, pattern, flag))
            .collect();
        Ok(self.join_tags(possibilities_tags))
    }

    fn generate_proc_tag(&self) -> String {
        // Format 'some_name' => 'name="some_name"' if set
        let name_string = self.to_attribute("name", &self.name);
        // Format '56' to 'chance="56"'
        let chance_string = self.to_attribute("chance", &self.chance);
        // Format 'spd' to 'stat="spd"'
        let stat_string = self.to_attribute("stat", &self.stat);

        self.to_attribute_tag("procedural", vec![name_string, chance_string, stat_string])
    }

    fn generate_poss_tags(&self) -> String {
        let mut poss_string: Vec<String> = Vec::new();
        for (name, chance) in self.possibilities() {
            let mut tag: Vec<String> = Vec::new();
            let name_string = self.to_attribute("name", name);
            let chance_string = self.to_attribute("chance", chance);

            // Push opening tag
            tag.push(self.to_attribute_tag("poss", vec![name_string, chance_string]));
            // Push name if exists
            if let Some(name) = name
                && !name.is_empty()
            {
                tag.push(name.to_owned());
            }
            // Push closing tag
            tag.push("</poss>".into());

            // Append into main string
            poss_string.append(&mut tag);
        }

        poss_string.join("")
    }

    fn to_attribute<T: ToString>(&self, attr_name: &str, attribute: &Option<T>) -> String {
        match attribute {
            Some(a) => {
                let attr = a.to_string();
                format!("{attr_name}=\"{attr}\"")
            }
            None => String::new(),
        }
    }

    fn to_attribute_tag(&self, tag_name: &str, attributes: Vec<String>) -> String {
        let mut tag: Vec<String> = Vec::new();
        // Add opening brace and tag name
        tag.push(format!("<{tag_name}"));
        // Append attributes
        for value in attributes {
            if !value.is_empty() {
                tag.push(format!(" {value}"))
            };
        }
        // Add closing brace
        tag.push(">".into());

        tag.join("")
    }

    fn to_possible_phrase_tag(
        &self,
        tag_name: &str,
        attribute: &str,
        pattern: &str,
        flag: PossibleFlag,
    ) -> String {
        let transformed = self.transform_possible_attribute(attribute, flag);
        let replaced_phrase = pattern.replace("{}", &transformed);
        format!("[{tag_name}={attribute}: {replaced_phrase}]")
    }

    fn to_possible_name_tag(&self, tag_name: &str, attribute: &str, flag: PossibleFlag) -> String {
        let transformed = self.transform_possible_attribute(attribute, flag);
        format!("[{tag_name}={attribute}: {transformed}]")
    }

    fn transform_possible_attribute(&self, attribute: &str, flag: PossibleFlag) -> String {
        match flag {
            PossibleFlag::Uppercase => attribute.to_uppercase(),
            PossibleFlag::Lowercase => attribute.to_lowercase(),
        }
    }

    fn join_tags(&self, tags: Vec<String>) -> String {
        let len = tags.len();
        tags.iter()
            .enumerate()
            .map(|(i, s)| {
                // If tag is not last tag
                if i != len - 1 {
                    // Add comma
                    s.to_owned() + ", "
                } else {
                    // Leave last tag as is
                    s.to_owned()
                }
            })
            .collect::<String>()
    }

    fn get_named_possibilities(&self) -> Result<Vec<String>> {
        // filter into where name is present and discard chance
        let some: Vec<String> = self
            .possibilities
            .clone()
            .into_iter()
            .filter_map(|(name, _)| name)
            .collect();
        // If no possibilities are named, bail
        if some.is_empty() {
            bail!("At least one possibility must be named.")
        }
        Ok(some)
    }

    fn get_some_name(&self) -> Result<&str> {
        if let Some(name) = &self.name {
            Ok(name)
        } else {
            bail!("Procedural must be named.")
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProceduralBuilder {
    name: Option<String>,
    chance: Option<f64>,
    stat: Option<Stat>,
    possibilities: Vec<Possibility>,
}

impl ProceduralBuilder {
    pub fn new() -> ProceduralBuilder {
        ProceduralBuilder::default()
    }

    pub fn name(&mut self, name: &str) -> &mut ProceduralBuilder {
        self.name = Some(name.to_owned());
        self
    }

    pub fn chance(&mut self, chance: f64) -> &mut ProceduralBuilder {
        self.chance = Some(chance);
        self
    }

    pub fn stat(&mut self, stat: Stat) -> &mut ProceduralBuilder {
        self.stat = Some(stat);
        self
    }

    pub fn possibility(
        &mut self,
        name: Option<&str>,
        chance: Option<f64>,
    ) -> &mut ProceduralBuilder {
        self.possibilities
            .push((name.map(|n| n.to_owned()), chance));
        self
    }

    pub fn possibilities(
        &mut self,
        possibilities: Vec<(Option<&str>, Option<f64>)>,
    ) -> &mut ProceduralBuilder {
        self.possibilities.extend(
            possibilities
                .iter()
                .map(|(name, chance)| (name.map(|n| n.to_owned()), chance.to_owned())),
        );
        self
    }

    pub fn build(&self) -> Result<Procedural> {
        if let Some(c) = self.chance {
            if !self.validate_chance(&c) {
                bail!("Chance {c} cannot be more than 100 or be less than 0.");
            }
        }

        for (_, chance) in &self.possibilities {
            if let Some(c) = chance {
                if !self.validate_chance(c) {
                    bail!("Possibility {c} cannot be more than 100 or be less than 0.");
                }
            }
        }

        Ok(Procedural {
            name: self.name.clone(),
            chance: self.chance,
            stat: self.stat.clone(),
            possibilities: self.possibilities.clone(),
        })
    }

    fn validate_chance(&self, chance: &f64) -> bool {
        if chance > &100.0 || chance < &0.0 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_valid() {
        let mut builder = ProceduralBuilder::new();
        builder.name("Test");
        builder.chance(64.0);
        builder.possibility(Some("Test"), Some(34.0));
        builder.stat(Stat::Dexterity);
        let result = builder.build();
        let expected = Procedural {
            name: Some("Test".into()),
            chance: Some(64.0),
            stat: Some(Stat::Dexterity),
            possibilities: vec![(Some("Test".into()), Some(34.0))],
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_builder_err() {
        let mut builder = ProceduralBuilder::new();
        builder.chance(132.0);
        let chance = builder.build();
        assert!(chance.is_err());

        let possibility = Procedural::builder().possibility(None, Some(-64.0)).build();
        assert!(possibility.is_err());

        let possibilities = Procedural::builder()
            .possibilities(vec![
                (None, Some(145.0)),
                (Some("Test"), Some(0.0)),
                (None, None),
            ])
            .build();
        assert!(possibilities.is_err());
    }

    #[test]
    fn test_empty_procedural_generate() {
        let proc = Procedural::builder().build();
        let expected = String::from("<procedural></procedural>");
        let output = proc.unwrap().generate_procedural();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_procedural_generate() {
        let mut builder = Procedural::builder();
        builder.name("beverage flavor");
        builder.chance(100.0);
        builder.stat(Stat::Speed);
        builder.possibility(Some("water"), Some(66.6));
        builder.possibility(Some("crush"), None);
        builder.possibility(Some("sierra mist"), None);
        builder.possibility(Some("root beer"), None);
        let result = builder.build().unwrap().generate_procedural();
        let expected = String::from(
            r#"<procedural name="beverage flavor" chance="100" stat="spd"><poss name="water" chance="66.6">water</poss><poss name="crush">crush</poss><poss name="sierra mist">sierra mist</poss><poss name="root beer">root beer</poss></procedural>"#,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_possible_names() {
        let mut builder = Procedural::builder();
        builder.name("beverage flavor");
        builder.possibility(Some("water"), Some(66.6));
        builder.possibility(Some("crush"), None);
        builder.possibility(Some("sierra mist"), None);
        builder.possibility(Some("root beer"), None);
        let uppercase_result = builder
            .build()
            .unwrap()
            .generate_possible_names(PossibleFlag::Uppercase)
            .unwrap();
        let lowercase_result = builder
            .build()
            .unwrap()
            .generate_possible_names(PossibleFlag::Lowercase)
            .unwrap();
        let uppercase_expected = "[beverage flavor=water: WATER], [beverage flavor=crush: CRUSH], [beverage flavor=sierra mist: SIERRA MIST], [beverage flavor=root beer: ROOT BEER]";
        let lowercase_expected = "[beverage flavor=water: water], [beverage flavor=crush: crush], [beverage flavor=sierra mist: sierra mist], [beverage flavor=root beer: root beer]";

        assert_eq!(uppercase_result, uppercase_expected);
        assert_eq!(lowercase_result, lowercase_expected);
    }

    #[test]
    fn test_possible_containing_phrases() {
        let mut builder = Procedural::builder();
        builder.name("beverage flavor");
        builder.possibility(Some("water"), Some(66.6));
        builder.possibility(Some("crush"), None);
        builder.possibility(Some("sierra mist"), None);
        builder.possibility(Some("root beer"), None);
        let pattern = "a bottle of {}, bottles of {}";
        let uppercase_result = builder
            .build()
            .unwrap()
            .generate_possible_containing_phrases(pattern, PossibleFlag::Uppercase)
            .unwrap();
        let lowercase_result = builder
            .build()
            .unwrap()
            .generate_possible_containing_phrases(pattern, PossibleFlag::Lowercase)
            .unwrap();
        let uppercase_expected = "[beverage flavor=water: a bottle of WATER, bottles of WATER], [beverage flavor=crush: a bottle of CRUSH, bottles of CRUSH], [beverage flavor=sierra mist: a bottle of SIERRA MIST, bottles of SIERRA MIST], [beverage flavor=root beer: a bottle of ROOT BEER, bottles of ROOT BEER]";
        let lowercase_expected = "[beverage flavor=water: a bottle of water, bottles of water], [beverage flavor=crush: a bottle of crush, bottles of crush], [beverage flavor=sierra mist: a bottle of sierra mist, bottles of sierra mist], [beverage flavor=root beer: a bottle of root beer, bottles of root beer]";

        assert_eq!(uppercase_result, uppercase_expected);
        assert_eq!(lowercase_result, lowercase_expected);
    }
}
