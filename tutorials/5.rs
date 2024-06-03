// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        let dp = target() - position();
        let target_future_location = target() + (target_velocity()*2.0 * (dp.length()/BULLET_SPEED));
        debug!("distance to target: {}", dp.length());
        debug!("time to target: {}", dp.length() / BULLET_SPEED);
        debug!("target location: {}", target());
        debug!("target future location: {}", target_future_location);
        draw_line(position(), target(), 0x00ff00);
        draw_line(position(), target_future_location, 0x0000FF);
        draw_line(target(), target_future_location, 0xFF0000);
        debug!("target velocity: {}", target_velocity());

        turn(angle_diff(heading(), (target_future_location-position()).angle()));
        fire(0);
    }
}
