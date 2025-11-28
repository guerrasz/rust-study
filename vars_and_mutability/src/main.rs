#![allow(unused_variables)]

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "Summer";

    let mut points_scored: i32 = 28;
    points_scored += 7;

    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "The season is: {}, the event happened at: 0{}:00, and the team scored {points_scored} points. Each touchdown is worth {2} points.",
        season, event_time, TOUCHDOWN_POINTS
    );

    let favorite_beverage: &str = "Coconut water";
}
