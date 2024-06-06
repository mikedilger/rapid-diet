use crate::{Nutrition, Food};

pub fn sardines(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/100080908/100g/1
    let n100 = Nutrition {
        energy_kj: 235.0 * 4.184,
        fat_g: 14.1,
        protein_g: 25.9,
        carbs_g: 0.0,
        sodium_mg: 400.0,
        cholesterol_mg: 65.9,
        fiber_g: 0.0,
        n3_fat_g: (0.982 / 11.5) * 14.1,
        n6_fat_g: (3.543 / 11.5) * 14.1,
        lysine_mg: (2260.0 / 24.6) * 25.9,
        isoleucine_mg: (1134.0 / 24.6) * 25.9,
        leucine_mg: (2001.0 / 24.6) * 25.9,
        methionine_mg: (729.0 / 24.6) * 25.9,   // 40%
        phenylalanine_mg: (961.0 / 24.6) * 25.9,
        tryptophan_mg: (276.0 / 24.6) * 25.9,
        valine_mg: (1268.0 / 24.6) * 25.9,
        threonine_mg: (1079.0 / 24.6) * 25.9,
        // from: https://tools.myfooddata.com/nutrition-facts/175139/100g/1
        folate_ug: 10.0,
        niacin_mg: 5.2,
        pantothenic_acid_mg: 0.64, // 12.8%
        riboflavin_mg: 0.23,
        thiamin_mg: 0.08,
        cobalamin_ug: 8.9,
        b6_mg: 0.17,
        vitamin_c: 0.0,
        vitamin_a_ug: 32.0,
        vitamin_d_ug: 4.8,
        vitamin_e_mg: 2.0,
        calcium_mg: 382.0,
        magnesium_mg: 39.0,
        zinc_mg: 1.3,
        copper_mg: 0.19,
        potassium_mg: 397.0,
        selenium_mcg: 52.7,
    };

    Food {
        name: "Sardines".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn rapeseed_oil(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/172336/100g/1
    let n100 = Nutrition {
        energy_kj: 884.0 * 4.184,
        fat_g: 100.0,
        protein_g: 0.0,
        carbs_g: 0.0,
        sodium_mg: 0.0,
        cholesterol_mg: 0.0,
        fiber_g: 0.0,
        n3_fat_g: 9.137,
        n6_fat_g: 18.64,
        lysine_mg: 0.0,
        isoleucine_mg: 0.0,
        leucine_mg: 0.0,
        methionine_mg: 0.0,
        phenylalanine_mg: 0.0,
        tryptophan_mg: 0.0,
        valine_mg: 0.0,
        threonine_mg: 0.0,
        folate_ug: 0.0,
        niacin_mg: 0.0,
        pantothenic_acid_mg: 0.0,
        riboflavin_mg: 0.0,
        thiamin_mg: 0.0,
        cobalamin_ug: 0.0,
        b6_mg: 0.0,
        vitamin_c: 0.0,
        vitamin_a_ug: 0.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 2.4,
        calcium_mg: 0.0,
        magnesium_mg: 0.0,
        zinc_mg: 0.0,
        copper_mg: 0.0,
        potassium_mg: 0.0,
        selenium_mcg: 0.0,
    };

    Food {
        name: "Rapeseed oil".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn riga_gold_with_oil(g: f32) -> Food {
    // 100g of pure sardines are 25.9g of protein
    // 100g of Riga Gold is only 16g of protein
    // So we assume Riga Gold is 61.8% sardines at 38.2% rapeseed oil
    //
    // Problem is we end up with too high energy (2020 vs 1456) and fat (46 vs 32)
    // THEY PROBABLY consider the fish only after it is drained.

    let s = sardines(61.8);
    let r = rapeseed_oil(38.2);
    Food {
        name: "Riga Gold".to_owned(),
        grams: g,
        nutrition: (s.nutrition + r.nutrition) * (g / 100.0),
    }
}

pub fn pea_protein_drink(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/100068990/100g/1
    let n100 = Nutrition {
        energy_kj: 1630.0,
        fat_g: 1.0,
        protein_g: 78.2,
        carbs_g: 5.4,
        sodium_mg: 1170.0,
        cholesterol_mg: 0.0,
        fiber_g: 3.7,
        n3_fat_g: 0.0,
        n6_fat_g: 0.0,
        lysine_mg: 5720.0,
        isoleucine_mg: 3620.0,
        leucine_mg: 6310.0,
        methionine_mg: 947.0,  // 49%
        phenylalanine_mg: 4200.0,
        tryptophan_mg: 683.0,
        valine_mg: 3920.0,
        threonine_mg: 3050.0,
        // FIXME, this is unknown data:
        // https://www.nutritionvalue.org/Unsweetened_pea_protein_powder_by_Bob%27s_Red_Mill_Natural_Foods%2C_Inc._571972_nutritional_value.html?size=100+g
        folate_ug: 0.0,
        niacin_mg: 0.0,
        pantothenic_acid_mg: 0.0,
        riboflavin_mg: 0.0,
        thiamin_mg: 0.0,
        cobalamin_ug: 0.0,
        b6_mg: 0.0,
        vitamin_c: 0.0,
        vitamin_a_ug: 0.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 0.0,
        calcium_mg: 74.0,
        magnesium_mg: 0.0, // unknown
        zinc_mg: 0.0, // unknnown
        copper_mg: 0.0, // unknown
        potassium_mg: 580.0,
        selenium_mcg: 0.0, // unknown
    };

    Food {
        name: "Pea Protein Drink".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn brussel_sprouts(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/170383/100g/1
    let n100 = Nutrition {
        energy_kj: 43.0 * 4.184,
        fat_g: 0.3,
        protein_g: 3.4,
        carbs_g: 9.0,
        sodium_mg: 25.0,
        cholesterol_mg: 0.0,
        fiber_g: 3.8,
        n3_fat_g: 0.099,
        n6_fat_g: 0.045,
        lysine_mg: 154.0,
        isoleucine_mg: 132.0,
        leucine_mg: 152.0,
        methionine_mg: 32.0,
        phenylalanine_mg: 98.0,
        tryptophan_mg: 37.0,
        valine_mg: 155.0,
        threonine_mg: 120.0,
        folate_ug: 61.0,
        niacin_mg: 0.75,
        pantothenic_acid_mg: 0.31,  // 6.2%
        riboflavin_mg: 0.09,
        thiamin_mg: 0.14,
        cobalamin_ug: 0.0,
        b6_mg: 0.22,
        vitamin_c: 85.0,
        vitamin_a_ug: 38.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 0.88,
        calcium_mg: 42.0,
        magnesium_mg: 23.0,
        zinc_mg: 0.42,
        copper_mg: 0.07,
        potassium_mg: 389.0,
        selenium_mcg: 1.6,
    };

    Food {
        name: "Brussel sprouts".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn black_beans(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/175187/100g/1
    let n100 = Nutrition {
        energy_kj: 130.0 * 4.184,
        fat_g: 0.35,
        protein_g: 8.2,
        carbs_g: 24.4,
        sodium_mg: 3.0,
        cholesterol_mg: 0.0,
        fiber_g: 8.3,
        n3_fat_g: 0.068,
        n6_fat_g: 0.081,
        lysine_mg: 562.0,
        isoleucine_mg: 361.0,
        leucine_mg: 653.0,
        methionine_mg: 123.0,
        phenylalanine_mg: 442.0,
        tryptophan_mg: 97.0,
        valine_mg: 428.0,
        threonine_mg: 344.0,
        folate_ug: 86.0,
        niacin_mg: 0.53,
        pantothenic_acid_mg: 0.26,  // 5.2%
        riboflavin_mg: 0.06,
        thiamin_mg: 0.23,
        cobalamin_ug: 0.0,
        b6_mg: 0.08,
        vitamin_c: 0.0,
        vitamin_a_ug: 0.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 0.87,
        calcium_mg: 55.0,
        magnesium_mg: 49.0,
        zinc_mg: 0.76,
        copper_mg: 0.27,
        potassium_mg: 433.0,
        selenium_mcg: 1.2,
    };

    Food {
        name: "Black beans".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn pumpkin(g: f32) -> Food {
    // GOOD SOURCE OF:  Niacin, Vitamin B6, Vitamin A, Fibre

    // https://www.foodcomposition.co.nz/search/food/X1122/nip
    let n100 = Nutrition {
        energy_kj: 338.0,
        fat_g: 1.9,
        protein_g: 1.8,
        carbs_g: 11.5,
        sodium_mg: 0.0,
        cholesterol_mg: 0.0,
        fiber_g: 5.0,
        n3_fat_g: 0.18,
        n6_fat_g: 0.33,
        lysine_mg: 0.0, // unknown
        isoleucine_mg: 0.0, // unknown
        leucine_mg: 0.0, // unknown
        methionine_mg: 0.0, // unknown
        phenylalanine_mg: 0.0, // unknown
        tryptophan_mg: 0.0, // unknown
        valine_mg: 0.0, // unknown
        threonine_mg: 0.0, // unknown
        folate_ug: 9.0,
        niacin_mg: 2.2,
        pantothenic_acid_mg: 0.0, // unknown
        riboflavin_mg: 0.15,
        thiamin_mg: 0.02,
        cobalamin_ug: 0.0,
        b6_mg: 0.36,
        vitamin_c: 5.5,
        vitamin_a_ug: 177.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 1.4,
        calcium_mg: 43.0,
        magnesium_mg: 28.0,
        zinc_mg: 0.36,
        copper_mg: 0.07,
        potassium_mg: 650.0,
        selenium_mcg: 0.0,
    };

    Food {
        name: "Pumpkin".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn almonds(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/170159/100g/1
    let n100 = Nutrition {
        energy_kj: 607.0 * 4.184,
        fat_g: 55.2,
        protein_g: 21.2,
        carbs_g: 17.7,
        sodium_mg: 1.0,
        cholesterol_mg: 0.0,
        fiber_g: 10.5,             // 38%
        n3_fat_g: 0.0,
        n6_fat_g: 13.519,
        lysine_mg: 600.0,
        isoleucine_mg: 691.0,
        leucine_mg: 1467.0,
        methionine_mg: 188.0,
        phenylalanine_mg: 1146.0,
        tryptophan_mg: 192.0,
        valine_mg: 798.0,
        threonine_mg: 677.0,
        folate_ug: 27.0,
        niacin_mg: 3.7,            // 23%
        pantothenic_acid_mg: 0.23, // 4.6%
        riboflavin_mg: 0.78,       // 60%
        thiamin_mg: 0.09,
        cobalamin_ug: 0.0,
        b6_mg: 0.12,
        vitamin_c: 0.0,
        vitamin_a_ug: 0.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 26.0,         // 173%
        calcium_mg: 291.0,          // 22%
        magnesium_mg: 274.0,        // 65%
        zinc_mg: 3.1,               // 28%
        copper_mg: 0.96,            // 106%
        potassium_mg: 0.699,
        selenium_mcg: 4.1,
    };

    Food {
        name: "Almonds".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

// dry roasted
pub fn sunflower_seeds(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/170563/100g/1
    let n100 = Nutrition {
        energy_kj: 582.0 * 4.184,
        fat_g: 49.8,
        protein_g: 19.3,
        carbs_g: 24.1,
        sodium_mg: 3.0,
        cholesterol_mg: 0.0,
        fiber_g: 11.1,
        n3_fat_g: 0.069,
        n6_fat_g: 32.782,
        lysine_mg: 795.0,
        isoleucine_mg: 967.0,
        leucine_mg: 1408.0,
        methionine_mg: 420.0,
        phenylalanine_mg: 992.0,
        tryptophan_mg: 295.0,
        valine_mg: 1116.0,
        threonine_mg: 788.0,
        folate_ug: 237.0,
        niacin_mg: 7.0,
        pantothenic_acid_mg: 7.0, // 141%
        riboflavin_mg: 0.25,
        thiamin_mg: 0.11,
        cobalamin_ug: 0.0,
        b6_mg: 0.8,
        vitamin_c: 1.4,
        vitamin_a_ug: 0.0,
        vitamin_d_ug: 0.0,
        vitamin_e_mg: 26.1,
        calcium_mg: 70.0,
        magnesium_mg: 129.0, // 31%
        zinc_mg: 5.3, // 48%
        copper_mg: 1.8, // 203%
        potassium_mg: 850.0, // 18%
        selenium_mcg: 79.3, // 144%
    };

    Food {
        name: "Sunflower Seeds".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn vitamin_d_pill(iu: usize) -> Food {
    let mut n100 = Nutrition::zero();
    n100.vitamin_d_ug = iu as f32 * 25.0 / 1000.0;

    Food {
        name: "Vitamin D Pill".to_owned(),
        grams: 1.0,
        nutrition: n100,
    }
}

// Lite milk (1.5%)
pub fn lite_milk(g: f32) -> Food {
    let n100 = Nutrition {
        energy_kj: 186.0,
        fat_g: 1.5,
        protein_g: 3.4,
        carbs_g: 4.4,
        sodium_mg: 34.0,
        cholesterol_mg: 7.8,
        fiber_g: 0.0,
        n3_fat_g: 0.01,
        n6_fat_g: 0.02,
        lysine_mg: 0.0, // unknown
        isoleucine_mg: 0.0, // unknown
        leucine_mg: 0.0, // unknown
        methionine_mg: 0.0, // unknown
        phenylalanine_mg: 0.0, // unknown
        tryptophan_mg: 0.0, // unknown
        valine_mg: 0.0, // unknown
        threonine_mg: 0.0, // unknown
        folate_ug: 20.0,
        niacin_mg: 0.81,
        pantothenic_acid_mg: 0.0, // unknown
        riboflavin_mg: 0.24,
        thiamin_mg: 0.05,
        cobalamin_ug: 0.33,
        b6_mg: 0.05,
        vitamin_c: 0.0,
        vitamin_a_ug: 16.0,
        vitamin_d_ug: 0.00,
        vitamin_e_mg: 0.00,
        calcium_mg: 120.0,
        magnesium_mg: 11.0,
        zinc_mg: 0.36,
        copper_mg: 0.00,
        potassium_mg: 158.0,
        selenium_mcg: 1.7,
    };

    Food {
        name: "Lite Milk".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}
