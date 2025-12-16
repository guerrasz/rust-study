use std::ops::Add;

#[derive(Debug)]
pub struct Lunch {
    pub cost: f64,
}

impl Add for Lunch {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}
