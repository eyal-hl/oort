use oort_api::prelude::*;

const BULLET_SPEED: f64 = 900.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
      
      if let Some(contact) = scan() {
          set_radar_heading(contact.position.angle());
          aim_at_target(contact);
          fire(0);
      }else{
          set_radar_heading(radar_heading() + radar_width());
      }
  }
}

fn aim_at_target(target: ScanResult){
    let dp = target.position - position();
    let angle = smart_aim(dp, target.velocity, BULLET_SPEED);
    turn(turn_controller(angle, heading(), 10.0, 0.1));
    fire(0);
}

fn smart_aim(target: Vec2, target_velocity: Vec2, bullet_speed: f64) -> f64 {
    let r_cross_v = target.x * target_velocity.y - target.y * target_velocity.x;
    let mag_r = f64::sqrt(target.x * target.x + target.y * target.y);
    let angle_adjust = f64::asin(r_cross_v / (mag_r * bullet_speed));

    return angle_adjust + f64::atan2(target.y, target.x);
}

fn turn_controller(angle: f64, heading: f64, kp: f64, kd: f64) -> f64 {
    let error = angle_diff(heading, angle);
    let derivative = -kd * error;
    let proportional = kp * error;
    return derivative + proportional;
}
