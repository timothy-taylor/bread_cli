use crate::flour::Flour;
use crate::yeast::Yeast;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: &'static str,
    pub flour: Vec<Flour>,
    pub salt: f64,
    pub water: f64,
    pub yeast: Yeast,
    pub enrichments: Option<Vec<(&'static str, f64)>>,
    _base: bool,
}

impl Recipe {
    pub fn new(
        name: &'static str,
        flour: Vec<Flour>,
        salt: f64,
        water: f64,
        yeast: Yeast,
        enrichments: Option<Vec<(&'static str, f64)>>,
    ) -> Self {
        // in the base method everything is in bakers percentages
        // flour = 100, everything is relative to that
        Self {
            name,
            flour,
            salt,
            water,
            yeast,
            enrichments,
            _base: true,
        }
    }

    pub fn remap(&self, grams: f64) -> Recipe {
        let flour: Vec<Flour> = self
            .flour
            .iter()
            .map(|f| match f {
                Flour::AP(x) => Flour::AP(grams * x * 0.01),
                Flour::Bread(x) => Flour::Bread(grams * x * 0.01),
                Flour::WW(x) => Flour::WW(grams * x * 0.01),
            })
            .collect();
        let yeast: Yeast = match self.yeast {
            Yeast::Instant(x) => Yeast::Instant(grams * x * 0.01),
            Yeast::Starter(x) => Yeast::Starter(grams * x * 0.01),
        };
        let enrichments: Option<Vec<(&str, f64)>> = match &self.enrichments {
            None => None,
            Some(x) => Some(x.iter().map(|e| (e.0, e.1 * 0.01 * grams)).collect()),
        };

        Recipe {
            name: self.name,
            flour,
            salt: self.salt * grams * 0.01,
            water: self.water * grams * 0.01,
            yeast,
            enrichments,
            _base: false,
        }
    }
}


