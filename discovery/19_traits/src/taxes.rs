pub trait Investment<T> {
    // getter for the amount value
    fn get_amount(&self) -> T;

    // using setter and getter to define shared behavior using common methods
    fn double_amount(&mut self);
}

pub trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.get_amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
pub struct Income {
    pub amount: f64,
}

impl Investment<f64> for Income {
    fn get_amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
pub struct Bonus {
    pub value: f64,
}

impl Investment<f64> for Bonus {
    fn get_amount(&self) -> f64 {
        self.value
    }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
}

#[derive(Debug)]
pub struct QualityTime {
    pub time: u32,
}

impl Investment<u32> for QualityTime {
    fn get_amount(&self) -> u32 {
        self.time
    }

    fn double_amount(&mut self) {
        self.time *= 2;
    }
}
