// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let max: f64 = 221.0 * speed as f64;
    if speed > 0 && speed <= 4 {
        max
    } else if speed <= 8 {
        max * 0.9
    } else if speed <= 10 {
        max * 0.77
    } else {
        panic!("Incorrect speed value!!")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let hourly = production_rate_per_hour(speed);
    println!("the hourly speed is: {}", hourly);
    println!("the hourly speed is: {}", hourly / 60.0);
    (hourly / 60.0) as u32
}
