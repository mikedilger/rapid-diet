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
    }
}

fn main() {
    let normal_daily = normal_daily_nutrition(100.0, 54.0, 1.81);

    // This diet consists of (daily):
    let diet = Food::new();
    //
    // black coffee
    //
    // MID DAY MEAL
    //   1T of metamucil to keep me moving
    //   1 multivitamin (every OTHER day to not get too much)
    //   TWO servings of pea protein isolate drink (4 scoops, 60g)
    let diet = diet.combine(pea_protein_drink(60.0), "Diet".to_owned());

    // DINNER
    //   1/4 of a jar of Riga Gold sardines with the rapeseed oil
    let diet = diet.combine(riga_gold_with_oil(270.0 / 4.0), "Diet".to_owned());
    //   100g of steamed brussel sprouts
    let diet = diet.combine(brussel_sprouts(100.0), "Diet".to_owned());
    //   100g of boiled black beans
    let diet = diet.combine(black_beans(100.0), "Diet".to_owned());

    // consider some raw sweet potato (orange kumura), or crown pumpkin
    // consider mustard greens

    println!("{:?}", diet.nutrition / normal_daily);
}

/*
w/o the vitamin pill:
NutritionPercent {
    energy: 0.24444279,       -- 25% of our normal needs, for weight loss
    fat: 0.3237035,           -- ok
    protein: 0.6264574,       -- this is PLENTY, giving 0.75 g/kg which is almost RDA
    carbs: 0.09030064,        -- ok (carbs aren't essential)
    sodium: 0.2562457,        -- well within limits
    cholesterol: 0.091633946, -- well within limits
    fiber: 0.34123674,        -- nice
    n3_fat: 1.8907697,        -- plenty
    n6_fat: 0.4817453,        -- ok
    lysine: 1.3527843,        -- enough
    isoleucine: 1.6647613,    -- enough
    leucine: 1.3023401,       -- enough
    methionine: 0.54914355,   -- LOW, however methionine is the best one to be low on.
    phenylalanine: 1.0551715, -- enough
    tryptophan: 1.3300354,    -- enough
    valine: 1.4549578,        -- enough
    threonine: 1.3839456,     -- enough
    folate: 0.37792876,       -- higher than 25%
    niacin: 0.21557374,       -- CLOSE to 25%
    pantothenic_acid: 0.1673952, -- CLOSE to 25%
    riboflavin: 0.18918808,   -- LOW
    thiamin: 0.3361433,       -- higher than 25%
    cobalamin: 1.546931,      -- plenty
    b6: 0.21818556,           -- CLOSE to 25%
    vitamin_c: 0.9444444,     -- plenty
    vitamin_a: 0.05705422,    -- LOW
    vitamin_d: 0.100116,      -- LOW
    vitamin_e: 0.21354268,    -- CLOSE to 25%
    calcium: 0.23134714,      -- CLOSE to 25%
    magnesium: 0.21016394,    -- CLOSE to 25%
    zinc: 0.15657227,         -- LOW
}
*/
