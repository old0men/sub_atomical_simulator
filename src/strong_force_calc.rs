use bevy::prelude::*;
use crate::constants::*;
use rand::Rng;


pub fn strong_force(
    direction: Vec3,
    distance: f32,
) -> Vec3 {

    let mason_list = [PION, NEUTRAL_PION, RHO, OMEGA];
    let mut chosen_mason: f64 = 0.0;

    let mut change: f32 = rand::rng().random_range(0.0..100.0);

    if change <= 90.0{
        let change1: f32 = rand::rng().random_range(1.0..3.0);
        if change1 <= 2.0 {
            chosen_mason = mason_list[0];
            println!("mason: 0");
        } else {
            chosen_mason = mason_list[1];
            println!("mason: 1");
        }
    } else if change <= 97.5{
        chosen_mason = mason_list[2];
        println!("mason: 2");
    } else {
        chosen_mason = mason_list[3];
        println!("mason: 3");
    }

    let strong_force = (-COUPLING_CONSTANT * COUPLING_CONSTANT)/(EULERS_NUMBER.powf(-chosen_mason*distance as f64)/distance as f64);

    let result = Vec3::new(direction.x*strong_force as f32 * 19e18, direction.y*strong_force as f32 * 19e18, 0.0);

    println!("strong force: {result}");
    result
}