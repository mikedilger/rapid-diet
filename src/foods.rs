use crate::{Nutrition, Food};

pub fn sardines(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/100080908/100g/1#modal
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
        methionine_mg: (729.0 / 24.6) * 25.9,
        phenylalanine_mg: (961.0 / 24.6) * 25.9,
        tryptophan_mg: (276.0 / 24.6) * 25.9,
        valine_mg: (1268.0 / 24.6) * 25.9,
        threonine_mg: (1079.0 / 24.6) * 25.9,
        // from: https://tools.myfooddata.com/nutrition-facts/175139/100g/1
        folate_ug: 10.0,
        niacin_mg: 5.2,
        pantothenic_acid_mg: 0.64,
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
    };

    Food {
        name: "Sardines".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn rapeseed_oil(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/172336/wt1
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
    // https://tools.myfooddata.com/nutrition-facts/100068990/wt1
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
        methionine_mg: 947.0,
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
        magnesium_mg: 0.0,
        zinc_mg: 0.0,
    };

    Food {
        name: "Pea Protein Drink".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn brussel_sprouts(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/170383/wt1
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
        pantothenic_acid_mg: 0.31,
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
    };

    Food {
        name: "Brussel sprouts".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}

pub fn black_beans(g: f32) -> Food {
    // https://tools.myfooddata.com/nutrition-facts/175187/wt1
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
        pantothenic_acid_mg: 0.26,
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
    };

    Food {
        name: "Black beans".to_owned(),
        grams: g,
        nutrition: n100 * (g / 100.0),
    }
}
