use anyhow::{Result, anyhow, bail};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Stat {
    Strength,
    Perception,
    Dexterity,
    Speed,
    Stamina,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Possibility {
    name: Option<String>,
    chance: Option<f64>,
}

impl Possibility {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn chance(&self) -> &Option<f64> {
        &self.chance
    }
}

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
}

#[derive(Debug, Default, Clone)]
pub struct ProceduralBuilder {
    name: Option<String>,
    chance: Option<f64>,
    stat: Option<Stat>,
    // possibilities: Vec<Possibility>,
    possibilities: Vec<(Option<String>, Option<f64>)>,
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

    pub fn stat(&mut self, stat: &Stat) -> &mut ProceduralBuilder {
        self.stat = Some(stat.to_owned());
        self
    }

    pub fn possibility(
        &mut self,
        name: &Option<String>,
        chance: &Option<f64>,
    ) -> &mut ProceduralBuilder {
        self.possibilities
            .push((name.to_owned(), chance.to_owned()));
        self
    }

    pub fn possibilities(
        &mut self,
        possibilities: &mut Vec<(Option<String>, Option<f64>)>,
    ) -> &mut ProceduralBuilder {
        self.possibilities.append(possibilities);
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
            possibilities: self
                .possibilities
                .iter()
                .map(|(name, chance)| Possibility {
                    name: name.to_owned(),
                    chance: chance.to_owned(),
                })
                .collect(),
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
        builder.possibility(&Some("Test".into()), &Some(34.0));
        builder.stat(&Stat::Dexterity);
        let result = builder.build();
        let expected = Procedural {
            name: Some("Test".into()),
            chance: Some(64.0),
            stat: Some(Stat::Dexterity),
            possibilities: vec![Possibility {
                name: Some("test".into()),
                chance: Some(34.0),
            }],
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_builder_err() {
        let mut builder = ProceduralBuilder::new();
        builder.chance(132.0);
        let should_error = builder.build();
        assert!(should_error.is_err());
    }
}
