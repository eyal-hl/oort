// Tutorial: Rotation
// Destroy the asteroid. The target is in a random
// location given by the "target()" function.
//
// You can get the angle between a vector and the x-axis: vec2(a, b).angle()
// And compare an angle with your ship's heading: angle_diff(heading(), angle)
//
// If angle_diff returns a positive number you need to turn left, or if negative then right.
// The turn function takes a speed argument, where positive speeds result in turning left
// and negative speeds will turn right.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        let dist = position().distance(target());
        debug!("{}",dist);
        // returns the direction your ship needs to turn to face the target.
        turn(angle_diff(heading(), (target() - position()).angle()));
        if angle_diff(heading(), (target() - position()).angle()).abs()< 0.1{
            accelerate(target()-position());
            fire(0);
        }
        if dist < 40.0{
            explode();
        }
    }
}
