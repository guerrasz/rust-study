pub mod cardio;
pub mod weightlifting;
pub mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {}", NUTRITIONIST);
    }
}

use cardio::{CardioTool, Exercise as CardioExercise};

use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    pub cardio: CardioExercise,
    pub weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        cardio::ask_about_program();
        weightlifting::ask_about_program();
        diet::ask_about_program();
        Self {
            cardio: CardioExercise::new(String::from("Monday"), CardioTool::Bike, 60),
            weightlifting: WeightliftingExercise::new(String::from("Bicep Curl"), 16),
        }
    }
}
