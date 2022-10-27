use crate::flour::Flour;
use crate::yeast::Yeast;
use crate::recipe::Recipe;

#[derive(Debug)]
pub struct HighAltiudeRecipe {
    pub name: &'static str,
    pub flour: Vec<Flour>,
    pub salt: f64,
    pub water: f64,
    pub yeast: Yeast,
    pub enrichments: Option<Vec<(&'static str, f64)>>,
    pub altitude: f64,
    _base: bool,
}

// the computations here are not accurate at this time,
// for your sake don't make this bread

impl HighAltiudeRecipe {
    pub fn new(altitude: f64, recipe: Recipe) -> Self {
        let flour: Vec<Flour> = recipe
            .flour
            .iter()
            .map(|f| match f {
                Flour::AP(x) => Flour::AP(flour_at_alt(*x, altitude)),
                Flour::Bread(x) => Flour::Bread(flour_at_alt(*x, altitude)),
                Flour::WW(x) => Flour::WW(flour_at_alt(*x, altitude)),
            })
            .collect();

        let water = liquid_at_alt(recipe.water, altitude);
        let yeast: Yeast = match recipe.yeast {
            Yeast::Instant(x) => Yeast::Instant(yeast_at_alt(x)),
            Yeast::Starter(x) => Yeast::Starter(yeast_at_alt(x)),
        };
        let enrichments: Option<Vec<(&str, f64)>> = match &recipe.enrichments {
            None => None,
            Some(x) => Some(
                x.iter()
                    .map(|e| (e.0, en_at_alt(e.0, e.1, altitude)))
                    .collect(),
            ),
        };

        Self {
            name: recipe.name,
            flour,
            salt: recipe.salt,
            water,
            yeast,
            enrichments,
            altitude,
            _base: false,
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

fn chem_leavener_at_alt(x: f64, alt: f64) -> f64 {
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

fn calc_leaven(x: f64, alt: f64, low: f64, med: f64, high: f64) -> f64 {
    // second step in calculating chemical leavener quantities
    let result = match alt as i64 {
        3000..=5000 => x * low,
        5001..=6500 => x * med,
        6501..=8000 => x * high,
        _ => x,
    };
    result
}

fn liquid_at_alt(liquid: f64, altitude: f64) -> f64 {
    const TBSP_LIQUID: f64 = 14.8;
    const TSP_HALF_LIQUID: f64 = 7.5;
    let remaining: f64 = altitude - 3000_f64;

    liquid + TBSP_LIQUID + (remaining.div_euclid(1000_f64) * TSP_HALF_LIQUID)
}

fn yeast_at_alt(yeast: f64) -> f64 {
    yeast - yeast * 0.25
}

fn flour_at_alt(flour: f64, altitude: f64) -> f64 {
    const TBSP_FLOUR: f64 = 7.8;
    let remaining: f64 = altitude - 3000_f64;

    flour + TBSP_FLOUR + (remaining.div_euclid(1500_f64) * TBSP_FLOUR)
}


