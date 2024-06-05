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
        let angle = smart_aim(dp, target_velocity(), BULLET_SPEED);

        turn(torque_controller(angle, heading(), 50.0, 0.01));

        fire(0);
    }
}

fn smart_aim(target: Vec2, target_velocity: Vec2, bullet_speed: f64) -> f64 {
    let r_cross_v = target.x * target_velocity.y - target.y * target_velocity.x;
    let mag_r = f64::sqrt(target.x * target.x + target.y * target.y);
    let angle_adjust = f64::asin(r_cross_v / (mag_r * bullet_speed));

    return angle_adjust + f64::atan2(target.y, target.x);
}

fn torque_controller(angle: f64, heading: f64, kp: f64, kd: f64) -> f64 {
    let error = angle_diff(heading, angle);
    let derivative = -kd * error;
    let proportional = kp * error;
    return derivative + proportional;
}