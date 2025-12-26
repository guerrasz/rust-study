#![allow(unused)]

use std::collections::HashSet;
trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

#[derive(Debug)]
struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    fn is_valid(&self) -> bool {
        self.vegetables.len() > 0
    }

    fn calories(&self) -> u32 {
        let mut total = 0;
        total += self.protein.calories();
        total += self.dressing.calories();
        for veggie in &self.vegetables {
            total += veggie.calories();
        }

        total
    }

    fn has_duplicate_vegetables(&self) -> bool {
        // can be turned into an one liner but would be kind of grotesque
        let original_len = self.vegetables.len();
        let no_duplicates = self
            .vegetables
            .iter()
            .collect::<HashSet<&Vegetable>>()
            .len();
        original_len != no_duplicates
    }
}

#[cfg(test)]
mod tests {
    use std::io::empty;

    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::{fixture, rstest};

    #[fixture]
    fn standard_salad() -> Salad {
        Salad::new(
            Protein::Steak,                               // 300 cal
            vec![Vegetable::Tomato, Vegetable::Cucumber], // 20 cal + 15 cal = 35 cal
            Dressing::Vinaigrette,                        // 120 cal
        )
        // 300 + 35 + 120
    }

    #[fixture]
    fn duplicate_veggies_salad() -> Salad {
        Salad::new(
            Protein::Steak,
            vec![Vegetable::Tomato, Vegetable::Tomato],
            Dressing::Vinaigrette,
        )
    }

    #[fixture]
    fn empty_veggies_salad() -> Salad {
        Salad::new(Protein::Steak, vec![], Dressing::Vinaigrette)
    }

    #[rstest]
    fn creates_new_salad_instance(standard_salad: Salad) {
        assert!(
            standard_salad.dressing == Dressing::Vinaigrette
                && standard_salad.protein == Protein::Steak
                && standard_salad.vegetables == vec![Vegetable::Tomato, Vegetable::Cucumber],
        )
    }

    #[rstest]
    fn is_valid_works(standard_salad: Salad, empty_veggies_salad: Salad) {
        assert_ne!(standard_salad.is_valid(), empty_veggies_salad.is_valid());
        assert_eq!(standard_salad.is_valid(), true);
        assert_eq!(empty_veggies_salad.is_valid(), false);
    }

    #[rstest]
    fn calculates_salad_total_calories_correctly(standard_salad: Salad) {
        assert_eq!(standard_salad.calories(), 455)
    }

    #[rstest]
    fn has_duplicates_identifies_duplicate_veggies(
        standard_salad: Salad,
        duplicate_veggies_salad: Salad,
    ) {
        assert_ne!(
            standard_salad.has_duplicate_vegetables(),
            duplicate_veggies_salad.has_duplicate_vegetables()
        );
        assert_eq!(standard_salad.has_duplicate_vegetables(), false);
        assert_eq!(duplicate_veggies_salad.has_duplicate_vegetables(), true);
    }
}
