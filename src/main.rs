use crate::flour::Flour;
use crate::recipe::Recipe;
use crate::yeast::Yeast;
use clap::Parser;

pub mod flour;
pub mod highaltitude;
pub mod recipe;
pub mod yeast;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 500.0)]
    grams: f64,
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
        Some(enrichments),           // Some() or None
    );

    // 3. map the base recipe to a specific weight of flour
    //
    // call remap with the weight in grams
    // of the amount of flour you want to use
    let new_recipe = base_recipe.remap(cli.grams);
    new_recipe.print_recipe();
}
