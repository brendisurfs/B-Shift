#![allow(unused)]

use std::f64::consts::PI;
use uom::si::f64::{AngularVelocity, Length, Power, Torque, Velocity};
use uom::si::length::meter;
use uom::si::power::{horsepower_metric, watt};
use uom::si::torque::newton_meter;
use uom::si::velocity::meter_per_second;

// Power is torque * rotational speed (rad/s)
// returns power in watts
pub fn power_in_watts(rpm: AngularVelocity, torque: Torque) -> Power {
    torque * (rpm * ((2.0 * PI) / 60.0))
}

// return wheel rpm in rad/s
pub fn wheel_angular_velocity(speed: Velocity, tire_radius: Length) -> AngularVelocity {
    (speed / tire_radius).into()
}

/// converts the output power to metric horsepower (for display only)
pub fn power_in_metric_hp(power: Power) -> f64 {
    power.get::<uom::si::power::horsepower_metric>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn watt_to_hp() {
        let w = wheel_angular_velocity(
            Velocity::new::<meter_per_second>(50.0),
            Length::new::<meter>(0.3),
        );

        let t = Torque::new::<newton_meter>(50.0);
        let p = power_in_watts(w, t);

        dbg!(p.ceil::<watt>());
    }
}
