pub mod flour;
pub mod yeast;
pub mod highaltitude;
pub mod recipe;

use clap::Parser;
use crate::highaltitude::HighAltiudeRecipe;
use crate::recipe::Recipe;
use crate::flour::Flour;
use crate::yeast::Yeast;

pub fn print_altitude_recipe(recipe: HighAltiudeRecipe) {
    println!("///////////////////////");
    println!("[{}]", recipe.name);

    println!("for altitude: {} feet", recipe.altitude as i64);
    println!("your rising times are going to be shorter");
    println!("a cold start may be beneficial");
    println!("remember to increase baking temperature by 15-25 degrees (F)");
    println!("and decrease baking time by 5-8 minutes per 30 minutes");

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

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 500.0)]
    grams: f64,
}

pub fn print_recipe(recipe: Recipe) {
    println!("///////////////////////");
    println!("[{}]", recipe.name);
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
    let cli = Args::parse();

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
    let new_recipe = base_recipe.remap(cli.grams);

    // 4. remap the mapped recipe to a specific altitude
    //
    // call remap with altitude with your alitude in feet
    // don't use remap for altitude at less than 3000 feet
    //let altiude_recipe = HighAltiudeRecipe::new(7000_f64, base_recipe.clone());

    print_recipe(new_recipe);
    //print_altitude_recipe(altiude_recipe);
}
