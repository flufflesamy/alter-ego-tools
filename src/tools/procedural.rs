use anyhow::{Result, bail};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Stat {
    Strength,
    Perception,
    Dexterity,
    Speed,
    Stamina,
}

impl Stat {
    /// Converts the given Stat to a short string.
    ///
    /// # Examples
    /// ```rs
    /// let stat_string = Stat::Perception.to_string();
    /// let per = String::from("per");
    ///
    /// assert_eq!(per, stat_string);
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Stat::Strength => "str".into(),
            Stat::Perception => "per".into(),
            Stat::Dexterity => "dex".into(),
            Stat::Speed => "spd".into(),
            Stat::Stamina => "sta".into(),
        }
    }
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
    pub fn generate(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        // Push <procedural...>
        parts.push(self.generate_proc_tag());
        // Push possibilities
        parts.push(self.generate_poss_tags());
        // Push closing </procedural>
        parts.push("</procedural>".into());
        parts.join("")
    }

    fn generate_proc_tag(&self) -> String {
        // Format 'some_name' => 'name="some_name"' if set
        let name_string = match &self.name {
            Some(n) => self.to_attribute("name", n),
            None => String::new(),
        };
        // Format '56' to 'chance="56"'
        let chance_string = match &self.chance {
            Some(c) => self.to_attribute("chance", &c.to_string()),
            None => String::new(),
        };
        // Format 'spd' to 'stat="spd"'
        let stat_string = match &self.stat {
            Some(s) => self.to_attribute("stat", &s.to_string()),
            None => String::new(),
        };

        self.to_attribute_tag("procedural", vec![name_string, chance_string, stat_string])
    }

    fn generate_poss_tags(&self) -> String {
        let mut poss_string: Vec<String> = Vec::new();
        for (name, chance) in self.possibilities() {
            let mut tag: Vec<String> = Vec::new();
            let name_string = match &name {
                Some(n) => self.to_attribute("name", n),
                None => String::new(),
            };
            let chance_string = match &chance {
                Some(c) => self.to_attribute("chance", &c.to_string()),
                None => String::new(),
            };

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

    fn to_attribute(&self, attr_name: &str, attribute: &str) -> String {
        format!("{attr_name}=\"{attribute}\"")
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
        name: Option<String>,
        chance: Option<f64>,
    ) -> &mut ProceduralBuilder {
        self.possibilities.push((name, chance));
        self
    }

    pub fn possibilities(
        &mut self,
        possibilities: Vec<(Option<String>, Option<f64>)>,
    ) -> &mut ProceduralBuilder {
        self.possibilities.extend(possibilities);
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
        builder.possibility(Some("Test".into()), Some(34.0));
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
                (Some("Test".into()), Some(0.0)),
                (None, None),
            ])
            .build();
        assert!(possibilities.is_err());
    }
}
