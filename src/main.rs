mod types;
use types::{Nutrition, Food};

mod foods;
use foods::*;

// This is set for Male, >19y, with physical activity level of 1.4 - 1.59
fn normal_daily_nutrition(weight_kg: f32, age_y: f32, height_m: f32) -> Nutrition {

    let phys_coeff = 1.11; // PAL 1.4 - 1.59
    let eer_kcal = 662.0 - (9.53 * age_y) + phys_coeff * ((15.91 * weight_kg) + (539.6 * height_m));
    let eer_kj = eer_kcal * 4.184;

    Nutrition {
        energy_kj: eer_kj,

        fat_g: (eer_kj * 0.3) / 37.0,  // 37 kJ / g      // 30% of diet
        protein_g: (eer_kj * 0.15) / 17.0, // 17 kJ / g  // 15% of diet, 1.2g / kg body weight
        carbs_g: (eer_kj * 0.55) / 17.0, // 17 kJ / g    // 55% of diet, 4.4g / kg body weight

        sodium_mg: 3500.0,
        cholesterol_mg: 300.0,
        fiber_g: 28.0 * ((eer_kj / 4.184) / 2000.0),   // based on 28g/day on 2000kCal diet

        n3_fat_g: 1.6,
        n6_fat_g: 14.0,

        lysine_mg: 38.0 * weight_kg,
        isoleucine_mg: 19.0 * weight_kg,
        leucine_mg: 42.0 * weight_kg,
        methionine_mg: 19.0 * weight_kg,
        phenylalanine_mg: 33.0 * weight_kg,
        tryptophan_mg: 5.0 * weight_kg,
        valine_mg: 24.0 * weight_kg,
        threonine_mg: 20.0 * weight_kg,
        folate_ug: 400.0,
        niacin_mg: 16.0,
        pantothenic_acid_mg: 5.0,
        riboflavin_mg: 1.3,
        thiamin_mg: 1.2,
        cobalamin_ug: 2.4,
        b6_mg: 1.7,
        vitamin_c: 90.0,
        vitamin_a_ug: 900.0,
        vitamin_d_ug: 20.0,
        vitamin_e_mg: 15.0,
        calcium_mg: 1300.0,
        magnesium_mg: 420.0,
        zinc_mg: 11.0,
        copper_mg: 0.9,
        potassium_mg: 3400.0,
        selenium_mcg: 55.0,
    }
}

fn main() {
    let normal_daily = normal_daily_nutrition(100.0, 54.0, 1.81);

    // This diet consists of (daily):
    let mut diet = Food::new();
    //
    // black coffee
    //
    // MID DAY MEAL
    //   1T of metamucil to keep me moving
    //   TWO servings of pea protein isolate drink (4 scoops, 60g)
    diet.combine(pea_protein_drink(60.0));

    // DINNER
    //   1/8 of a jar of Riga Gold sardines with the rapeseed oil
    diet.combine(riga_gold_with_oil(270.0 / 8.0));
    //   100g of steamed brussel sprouts
    diet.combine(brussel_sprouts(200.0));
    //   100g of boiled black beans
    diet.combine(black_beans(120.0));
    //   200g of baked crown pumpkin flesh
    diet.combine(pumpkin(200.0));
    //   20g of roasted unsalted almonds
    diet.combine(almonds(40.0));
    //   20g of roasted sunflower seeds
    diet.combine(sunflower_seeds(20.0));
    //   plus a VITAMIN D pill of 1000 UI
    diet.combine(vitamin_d_pill(1000));

    // Every other day take a multivitamin pill too

    println!("{:?}\n", diet.nutrition);
    println!("{}", diet.nutrition / normal_daily);
}

/*
 * Nutrition Percent:
 *   ENERGY: 38.7%   FAT 52.4%  PROTEIN 76.7%  CARBS 21.1%
 *   SODIUM 24.0%  CHOLESTEROL 4.6%
 *   fiber 86.3%  n3-fat 130.2%  n6-fat 115.1%
 *   lysine 139.7%  isoleucine 188.8%  leucine 147.2%  methionine: 57.8%
 *   phenylalanine 124.7%  tryptophan 159.3%  valine 166.5%  threonine: 157.4%
 *   B1 55.9%  B2 74.0%  B3 65.6%  B5 51.1%  B6 88.2%  B9 75.9%  B12 77.3%
 *   VitA 48.5%  VitC 201.4%  VitD 130.0%  VitE 146.3%
 *   Ca 37.7%  Mg 72.5%  Zn 45.8%  Cu 154.2%  K 94.1%  Se 60.2%
 *
 * ENERGY: 34.4% -- we get it from our adipose tissue instead
 * FAT: -- more than 100% of essential fat: n3-fat 123.1%  n6-fat 105.1%
 * PROTEIN: 70.3% -- 84 g/kg meets the RDA.
 * CARBS: -- there are no essential carbs. Our body will use the protein
 *           to generate more carbs
 * Ess Amino Acids:  We are only low in methionine, which is hard to find.
 * Vitamins: we are low in some. Take a multivitamin pill
 */
