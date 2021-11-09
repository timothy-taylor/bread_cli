use std::fmt;

#[derive(Debug)]
pub enum Yeast {
    Instant(f64),
    Starter(f64),
}

impl fmt::Display for Yeast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (s, x) = match self {
            Yeast::Instant(x) => ("instant yeast", x),
            Yeast::Starter(x) => ("sourdough starter", x),
        };
        write!(f, "{}: {}", s, x)
    }
}

#[derive(Debug)]
pub enum Flour {
    AP(f64),
    Bread(f64),
    WW(f64),
}

impl fmt::Display for Flour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (s, x) = match self {
            Flour::AP(x) => ("ap", x),
            Flour::Bread(x) => ("bread", x),
            Flour::WW(x) => ("whole wheat", x),
        };
        write!(f, "{} flour: {} grams", s, x)
    }
}

#[derive(Debug)]
pub struct Recipe {
    name: &'static str,
    flour: Vec<Flour>,
    salt: f64,
    water: f64,
    yeast: Yeast,
    enrichments: Option<Vec<(&'static str, f64)>>,
    _base: bool,
    high_altitude: Option<f64>,
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
            high_altitude: None,
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
            high_altitude: None,
        }
    }

    pub fn remap_for_altiude(&self, altitude: f64) -> Recipe {
        /*
        if altitude < 3000_f64 {
            return Recipe {
                name: self.name,
                flour: self.flour,
                salt: self.salt,
                water: self.water,
                yeast: self.yeast,
                enrichments: self.enrichments,
                _base: self._base,
                high_altitude: self.high_altitude,
            };
        }
        */

        let high_altitude: Option<f64> = Some(altitude);
        let flour: Vec<Flour> = self
            .flour
            .iter()
            .map(|f| match f {
                Flour::AP(x) => Flour::AP(flour_at_alt(*x, altitude)),
                Flour::Bread(x) => Flour::Bread(flour_at_alt(*x, altitude)),
                Flour::WW(x) => Flour::WW(flour_at_alt(*x, altitude)),
            })
            .collect();
        let water = liquid_at_alt(self.water, altitude);
        let yeast: Yeast = match self.yeast {
            Yeast::Instant(x) => Yeast::Instant(yeast_at_alt(x)),
            Yeast::Starter(x) => Yeast::Starter(yeast_at_alt(x)),
        };
        let enrichments: Option<Vec<(&str, f64)>> = match &self.enrichments {
            None => None,
            Some(x) => Some(
                x.iter()
                    .map(|e| (e.0, en_at_alt(e.0, e.1, altitude)))
                    .collect(),
            ),
        };

        Recipe {
            name: self.name,
            flour,
            salt: self.salt,
            water,
            yeast,
            enrichments,
            _base: false,
            high_altitude,
        }
    }
}

pub fn en_at_alt(name: &'static str, x: f64, alt: f64) -> f64 {
    const TBSP_SUGAR: f64 = 12.5;
    const EGG_GRAMS: f64 = 49.8;

    let new_value = match name {
        "eggs" => x + EGG_GRAMS,
        "sweetener" => x - TBSP_SUGAR * x.div_euclid(200_f64),
        "fat" => liquid_at_alt(x, alt),
        "milk" => liquid_at_alt(x, alt),
        "liquid" => liquid_at_alt(x, alt),
        "baking soda" => chem_leavener_at_alt(x, alt),
        "baking powder" => chem_leavener_at_alt(x, alt),
        _ => x,
    };
    new_value
}

pub fn chem_leavener_at_alt(x: f64, alt: f64) -> f64 {
    // for calculating chemimcal leavener quantities
    const TSP: f64 = 4.6;
    let new_value = match ((x / TSP) * 100.0) as i64 {
        0..=10 => calc_leaven(x, alt, 7.0 / 8.0, 0.5, 0.25),
        11..=20 => calc_leaven(x, alt, 0.75, 0.5, 1.0 / 3.0),
        21..=30 => calc_leaven(x, alt, 2.0 / 3.0, 1.25 / 3.0, 1.0 / 3.0),
        31..=40 => calc_leaven(x, alt, 0.625, 0.375, 0.25),
        _ => x,
    };
    new_value
}

pub fn calc_leaven(x: f64, alt: f64, low: f64, med: f64, high: f64) -> f64 {
    // second step in calculating chemical leavener quantities
    let result = match alt as i64 {
        3000..=5000 => x * low,
        5001..=6500 => x * med,
        6501..=8000 => x * high,
        _ => x,
    };
    result
}

pub fn liquid_at_alt(liquid: f64, altitude: f64) -> f64 {
    const TBSP_LIQUID: f64 = 14.8;
    const TSP_HALF_LIQUID: f64 = 7.5;
    let remaining: f64 = altitude - 3000_f64;

    liquid + TBSP_LIQUID + (remaining.div_euclid(1000_f64) * TSP_HALF_LIQUID)
}

pub fn yeast_at_alt(yeast: f64) -> f64 {
    yeast - yeast * 0.25
}

pub fn flour_at_alt(flour: f64, altitude: f64) -> f64 {
    const TBSP_FLOUR: f64 = 7.8;
    let remaining: f64 = altitude - 3000_f64;

    flour + TBSP_FLOUR + (remaining.div_euclid(1500_f64) * TBSP_FLOUR)
}

pub fn print_recipe(recipe: Recipe) {
    println!("///////////////////////");
    println!("[{}]", recipe.name);
    match recipe.high_altitude {
        None => (),
        Some(x) => {
            println!("for altitude: {} feet", x as i64);
            println!("your rising times are going to be shorter");
            println!("a cold start may be beneficial");
            println!("remember to increase baking temperature by 15-25 degrees (F)");
            println!("and decrease baking time by 5-8 minutes per 30 minutes");
        }
    }
    recipe.flour.into_iter().for_each(|f| println!("{}", f));
    println!("salt: {} grams", recipe.salt);
    println!("water: {} grams", recipe.water);
    println!("{} grams", recipe.yeast);
    match recipe.enrichments {
        None => (),
        Some(x) => x.iter().for_each(|e| println!("{}: {} grams", e.0, e.1)),
    };
    println!("///////////////////////");
}

fn main() {
    let mut enrichments = Vec::new();

    // 1. add your enrichments the vector
    //
    // enrichments beyond flour,water,salt,yeast
    // are added as vector of tuples (name, quantity)
    // quantity is given as a baker percentage (see step 2.)
    // if no enrichments than use None option
    // recommend using generic names
    // "fat", "sweetener",
    // for accuracy if using the high_altitude remapping
    // "baking soda" and "baking powder" are acceptable as strings
    enrichments.push(("fat", 8_f64));
    enrichments.push(("eggs", 4_f64));
    enrichments.push(("sweetener", 8_f64));
    enrichments.push(("powdered milk", 6_f64));

    // 2. make a base recipe with Recipe::new()
    //
    // base recipe should be in the form of
    // baker percentage https://en.wikipedia.org/wiki/Baker_percentage
    // flour quantities should always add up to 100
    // everything else is by weight in comparison
    let base_recipe = Recipe::new(
        "Pullman",
        vec![Flour::Bread(100_f64)], // 100% bread flour
        2_f64,                       // 2% salt
        58_f64,                      // 58% water
        Yeast::Instant(1_f64),       // 1% instant yeast
        Some(enrichments),
    );

    // 3. map the base recipe to a specific weight of flour
    //
    // call remap with the weight in grams
    // of the amount of flour you want to use
    let new_recipe = base_recipe.remap(500_f64);

    // 4. remap the mapped recipe to a specific altitude
    //
    // call remap with altitude with your alitude in feet
    // don't use remap for altitude at less than 3000 feet
    let altitude_recipe = new_recipe.remap_for_altiude(7000_f64);

    print_recipe(new_recipe);
    print_recipe(altitude_recipe);
}
