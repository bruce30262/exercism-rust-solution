use self::Allergen::*;

pub struct Allergies {
    score: u32,
}

//traits Clone, Copy are required
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGENS: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let ret:bool = if &self.score & *allergen as u32 != 0 { true } else { false };
        ret
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = ALLERGENS.to_vec();
        res.retain(|&a| self.is_allergic_to(&a));
        res
    }
}
