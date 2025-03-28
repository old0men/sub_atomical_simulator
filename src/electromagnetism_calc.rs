use bevy::prelude::Vec3;
use crate::constants::{ELEMENTARY_CHARGE, SCALE};


pub fn electrical_field(
    charge: f32,
    direction: Vec3,
    distance: f32,
) -> Vec3 {

    let charge_over_area = (charge*ELEMENTARY_CHARGE) / (distance.powf(2.0));
    println!("charge over area: {charge_over_area}");
    let normalized_unit_vector = direction.normalize();
    normalized_unit_vector * charge_over_area
}


pub fn magnetical_field(charge: f32,
                        speed_x: f32,
                        speed_y: f32,
                        direction: Vec3,
                        distance: f32,
) -> Vec3 {

    const PERMEABILITY: f32 = 10e-7f32;
    let charge_over_area = (charge*ELEMENTARY_CHARGE) / (distance.powf(2.0));
    let velocity_vector: Vec3 = Vec3::new(speed_x*SCALE, speed_y*SCALE, 0.0);

    //println!("magnetism-------------");
    let cross_product = cross_product(velocity_vector, direction.normalize());

    /*
    println!("unit dir calc :: x: ({}-{})/{}  y: ({}-{})/{}", x2, x1, distance, y2, y1, distance);
    println!("unit direction: {:?}", direction_unit_vector);
    println!("velocity vector {:?}", velocity_vector);
    println!("result :: charge over area {charge_over_area}, cross product x {} distance {distance}", cross_product.x);
     */

    let result = Vec3::new(0.0, 0.0,PERMEABILITY * charge_over_area * (cross_product.z / distance.powf(2.0)) );

    //println!("result {:?}", result);

    result
}

fn cross_product(a: Vec3, b: Vec3) -> Vec3 {

    //println!("{}*{}-{}*{}", a[0], b[1], b[0], a[1]);
    Vec3::new(a[1]*b[2]-b[1]*a[2],
              -(a[0]*b[2]-b[0]*a[2]),
              a[0]*b[1]-b[0]*a[1],)
}

pub fn loretz_force(
    charge: f32,
    electrical_field: Vec3,
    velocity_vector: Vec3,
    magnetic_field: Vec3,
) -> Vec3 {

    //println!("Lorentz------");
    //println!("Ef: {electrical_field}, Bf: {magnetic_field}, v: {velocity_vector}");

    let velocity_magnetic_field_cross_product = cross_product(velocity_vector*SCALE, magnetic_field);

    let inner_term = electrical_field+velocity_magnetic_field_cross_product;

    charge*inner_term/10000.0
}