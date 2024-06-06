use std::fmt;

use std::ops::{Add, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Nutrition {
    pub energy_kj: f32,

    pub fat_g: f32,
    pub protein_g: f32,
    pub carbs_g: f32,

    pub sodium_mg: f32,
    pub cholesterol_mg: f32,
    pub fiber_g: f32,

    pub n3_fat_g: f32,
    pub n6_fat_g: f32,

    pub lysine_mg: f32,
    pub isoleucine_mg: f32,
    pub leucine_mg: f32,
    pub methionine_mg: f32,
    pub phenylalanine_mg: f32,
    pub tryptophan_mg: f32,
    pub valine_mg: f32,
    pub threonine_mg: f32,

    pub folate_ug: f32,
    pub niacin_mg: f32,
    pub pantothenic_acid_mg: f32,
    pub riboflavin_mg: f32,
    pub thiamin_mg: f32,
    pub cobalamin_ug: f32,
    pub b6_mg: f32,
    pub vitamin_c: f32,

    pub vitamin_a_ug: f32,
    pub vitamin_d_ug: f32,
    pub vitamin_e_mg: f32,

    pub calcium_mg: f32,
    pub magnesium_mg: f32,
    pub zinc_mg: f32,
    pub copper_mg: f32,
    pub potassium_mg: f32,
    pub selenium_mcg: f32,
}

impl Nutrition {
    pub fn zero() -> Nutrition {
        Nutrition {
            energy_kj: 0.0,
            fat_g: 0.0,
            protein_g: 0.0,
            carbs_g: 0.0,
            sodium_mg: 0.0,
            cholesterol_mg: 0.0,
            fiber_g: 0.0,
            n3_fat_g: 0.0,
            n6_fat_g: 0.0,
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
            vitamin_e_mg: 0.0,
            calcium_mg: 0.0,
            magnesium_mg: 0.0,
            zinc_mg: 0.0,
            copper_mg: 0.0,
            potassium_mg: 0.0,
            selenium_mcg: 0.0,
        }
    }
}

impl Add for Nutrition {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Nutrition {
            energy_kj: self.energy_kj + other.energy_kj,
            fat_g: self.fat_g + other.fat_g,
            protein_g: self.protein_g + other.protein_g,
            carbs_g: self.carbs_g + other.carbs_g,
            sodium_mg: self.sodium_mg + other.sodium_mg,
            cholesterol_mg: self.cholesterol_mg + other.cholesterol_mg,
            fiber_g: self.fiber_g + other.fiber_g,
            n3_fat_g: self.n3_fat_g + other.n3_fat_g,
            n6_fat_g: self.n6_fat_g + other.n6_fat_g,
            lysine_mg: self.lysine_mg + other.lysine_mg,
            isoleucine_mg: self.isoleucine_mg + other.isoleucine_mg,
            leucine_mg: self.leucine_mg + other.leucine_mg,
            methionine_mg: self.methionine_mg + other.methionine_mg,
            phenylalanine_mg: self.phenylalanine_mg + other.phenylalanine_mg,
            tryptophan_mg: self.tryptophan_mg + other.tryptophan_mg,
            valine_mg: self.valine_mg + other.valine_mg,
            threonine_mg: self.threonine_mg + other.threonine_mg,
            folate_ug: self.folate_ug + other.folate_ug,
            niacin_mg: self.niacin_mg + other.niacin_mg,
            pantothenic_acid_mg: self.pantothenic_acid_mg + other.pantothenic_acid_mg,
            riboflavin_mg: self.riboflavin_mg + other.riboflavin_mg,
            thiamin_mg: self.thiamin_mg + other.thiamin_mg,
            cobalamin_ug: self.cobalamin_ug + other.cobalamin_ug,
            b6_mg: self.b6_mg + other.b6_mg,
            vitamin_c: self.vitamin_c + other.vitamin_c,
            vitamin_a_ug: self.vitamin_a_ug + other.vitamin_a_ug,
            vitamin_d_ug: self.vitamin_d_ug + other.vitamin_d_ug,
            vitamin_e_mg:self.vitamin_e_mg + other.vitamin_e_mg,
            calcium_mg: self.calcium_mg + other.calcium_mg,
            magnesium_mg: self.magnesium_mg + other.magnesium_mg,
            zinc_mg: self.zinc_mg + other.zinc_mg,
            copper_mg: self.copper_mg + other.copper_mg,
            potassium_mg: self.potassium_mg + other.potassium_mg,
            selenium_mcg: self.selenium_mcg + other.selenium_mcg,
        }
    }
}

impl Mul<f32> for Nutrition {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Nutrition {
            energy_kj: self.energy_kj * rhs,
            fat_g: self.fat_g * rhs,
            protein_g: self.protein_g * rhs,
            carbs_g: self.carbs_g * rhs,
            sodium_mg: self.sodium_mg * rhs,
            cholesterol_mg: self.cholesterol_mg * rhs,
            fiber_g: self.fiber_g * rhs,
            n3_fat_g: self.n3_fat_g * rhs,
            n6_fat_g: self.n6_fat_g * rhs,
            lysine_mg: self.lysine_mg * rhs,
            isoleucine_mg: self.isoleucine_mg * rhs,
            leucine_mg: self.leucine_mg * rhs,
            methionine_mg: self.methionine_mg * rhs,
            phenylalanine_mg: self.phenylalanine_mg * rhs,
            tryptophan_mg: self.tryptophan_mg * rhs,
            valine_mg: self.valine_mg * rhs,
            threonine_mg: self.threonine_mg * rhs,
            folate_ug: self.folate_ug * rhs,
            niacin_mg: self.niacin_mg * rhs,
            pantothenic_acid_mg: self.pantothenic_acid_mg * rhs,
            riboflavin_mg: self.riboflavin_mg * rhs,
            thiamin_mg: self.thiamin_mg * rhs,
            cobalamin_ug: self.cobalamin_ug * rhs,
            b6_mg: self.b6_mg * rhs,
            vitamin_c: self.vitamin_c * rhs,
            vitamin_a_ug: self.vitamin_a_ug * rhs,
            vitamin_d_ug: self.vitamin_d_ug * rhs,
            vitamin_e_mg: self.vitamin_e_mg * rhs,
            calcium_mg: self.calcium_mg * rhs,
            magnesium_mg: self.magnesium_mg * rhs,
            zinc_mg: self.zinc_mg * rhs,
            copper_mg: self.copper_mg * rhs,
            potassium_mg: self.potassium_mg * rhs,
            selenium_mcg: self.selenium_mcg * rhs,
        }
    }
}

impl Div<Nutrition> for Nutrition {
    type Output = NutritionPercent;

    fn div(self, other: Nutrition) -> NutritionPercent {
        NutritionPercent {
            energy: self.energy_kj / other.energy_kj,
            fat: self.fat_g / other.fat_g,
            protein: self.protein_g / other.protein_g,
            carbs: self.carbs_g / other.carbs_g,
            sodium: self.sodium_mg / other.sodium_mg,
            cholesterol: self.cholesterol_mg / other.cholesterol_mg,
            fiber: self.fiber_g / other.fiber_g,
            n3_fat: self.n3_fat_g / other.n3_fat_g,
            n6_fat: self.n6_fat_g / other.n6_fat_g,
            lysine: self.lysine_mg / other.lysine_mg,
            isoleucine: self.isoleucine_mg / other.isoleucine_mg,
            leucine: self.leucine_mg / other.leucine_mg,
            methionine: self.methionine_mg / other.methionine_mg,
            phenylalanine: self.phenylalanine_mg / other.phenylalanine_mg,
            tryptophan: self.tryptophan_mg / other.tryptophan_mg,
            valine: self.valine_mg / other.valine_mg,
            threonine: self.threonine_mg / other.threonine_mg,
            folate: self.folate_ug / other.folate_ug,
            niacin: self.niacin_mg / other.niacin_mg,
            pantothenic_acid: self.pantothenic_acid_mg / other.pantothenic_acid_mg,
            riboflavin: self.riboflavin_mg / other.riboflavin_mg,
            thiamin: self.thiamin_mg / other.thiamin_mg,
            cobalamin: self.cobalamin_ug / other.cobalamin_ug,
            b6: self.b6_mg / other.b6_mg,
            vitamin_c: self.vitamin_c / other.vitamin_c,
            vitamin_a: self.vitamin_a_ug / other.vitamin_a_ug,
            vitamin_d: self.vitamin_d_ug / other.vitamin_d_ug,
            vitamin_e: self.vitamin_e_mg / other.vitamin_e_mg,
            calcium: self.calcium_mg / other.calcium_mg,
            magnesium: self.magnesium_mg / other.magnesium_mg,
            zinc: self.zinc_mg / other.zinc_mg,
            copper: self.copper_mg / other.copper_mg,
            potassium: self.potassium_mg / other.potassium_mg,
            selenium: self.selenium_mcg / other.selenium_mcg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NutritionPercent {
    pub energy: f32,
    pub fat: f32,
    pub protein: f32,
    pub carbs: f32,
    pub sodium: f32,
    pub cholesterol: f32,
    pub fiber: f32,
    pub n3_fat: f32,
    pub n6_fat: f32,
    pub lysine: f32,
    pub isoleucine: f32,
    pub leucine: f32,
    pub methionine: f32,
    pub phenylalanine: f32,
    pub tryptophan: f32,
    pub valine: f32,
    pub threonine: f32,
    pub folate: f32, // B9
    pub niacin: f32, // B3
    pub pantothenic_acid: f32, // B5
    pub riboflavin: f32, // B2
    pub thiamin: f32, // B1
    pub cobalamin: f32, // B12
    pub b6: f32, // B6
    pub vitamin_c: f32,
    pub vitamin_a: f32,
    pub vitamin_d: f32,
    pub vitamin_e: f32,
    pub calcium: f32,
    pub magnesium: f32,
    pub zinc: f32,
    pub copper: f32,
    pub potassium: f32,
    pub selenium: f32,
}

impl fmt::Display for NutritionPercent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nutrition Percent: \n")?;
        write!(f, "  ENERGY: {:.1}%   FAT {:.1}%  PROTEIN {:.1}%  CARBS {:.1}%\n",
               self.energy * 100.0, self.fat * 100.0, self.protein * 100.0, self.carbs * 100.0)?;
        write!(f, "  SODIUM {:.1}%  CHOLESTEROL {:.1}%\n",
               self.sodium * 100.0, self.cholesterol * 100.0)?;
        write!(f, "  fiber {:.1}%  n3-fat {:.1}%  n6-fat {:.1}%\n",
               self.fiber * 100.0, self.n3_fat * 100.0, self.n6_fat * 100.0)?;
        write!(f, "  lysine {:.1}%  isoleucine {:.1}%  leucine {:.1}%  methionine: {:.1}%\n",
               self.lysine * 100.0, self.isoleucine * 100.0, self.leucine * 100.0, self.methionine * 100.0)?;
        write!(f, "  phenylalanine {:.1}%  tryptophan {:.1}%  valine {:.1}%  threonine: {:.1}%\n",
               self.phenylalanine * 100.0, self.tryptophan * 100.0, self.valine * 100.0, self.threonine * 100.0)?;
        write!(f, "  B1 {:.1}%  B2 {:.1}%  B3 {:.1}%  B5 {:.1}%  B6 {:.1}%  B9 {:.1}%  B12 {:.1}%\n",
               self.thiamin * 100.0, self.riboflavin * 100.0, self.niacin * 100.0,
               self.pantothenic_acid * 100.0, self.b6 * 100.0, self.folate * 100.0,
               self.cobalamin * 100.0)?;
        write!(f, "  VitA {:.1}%  VitC {:.1}%  VitD {:.1}%  VitE {:.1}%\n",
               self.vitamin_a * 100.0, self.vitamin_c * 100.0, self.vitamin_d * 100.0,
               self.vitamin_e * 100.0)?;
        write!(f, "  Ca {:.1}%  Mg {:.1}%  Zn {:.1}%  Cu {:.1}%  K {:.1}%  Se {:.1}%\n",
               self.calcium * 100.0, self.magnesium * 100.0, self.zinc * 100.0,
               self.copper * 100.0, self.potassium * 100.0, self.selenium * 100.0)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub grams: f32,
    pub nutrition: Nutrition,
}

impl Food {
    pub fn new() -> Food {
        Food {
            name: "(nothing)".to_owned(),
            grams: 0.0,
            nutrition: Nutrition::zero()
        }
    }

    pub fn combine(&mut self, other: Food) {
        self.grams += other.grams;
        self.nutrition = self.nutrition + other.nutrition;
    }
}
