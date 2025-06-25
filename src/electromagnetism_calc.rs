use bevy::color::palettes::basic::GREEN;
use bevy::math::Vec2;
use bevy::prelude::{Gizmos, Query, Transform, Vec3};
use crate::constants::{COULOMBS_CONSTANT, ELEMENTARY_CHARGE, SCALE};
use crate::{Movement, Particle};

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

    charge*inner_term
}




//----------------------------------------------------------------------------------------------------------------



fn acceleration_direction_em(from_x1: f32, from_y1: f32, to_x1: f32, to_y1: f32, distance: f32, charge1: f32, charge2: f32) -> Vec3 {
    let result = Vec3::new((to_x1-from_x1)/(distance/ SCALE),
                           (to_y1-from_y1)/(distance/ SCALE),
                           0.0);
    if charge2 == 0.0 || charge1 == 0.0 {
        Vec3::new(0.0,0.0,0.0)
    } else if charge1 == charge2 {
        -result
    } else {
        result
    }
}

fn distance(
    from_x1: f32, from_y1: f32, to_x1: f32, to_y1: f32
) -> f32 {
    (   (  (to_x1 - from_x1).powf(2.0)  +  (to_y1 - from_y1).powf(2.0) ).sqrt()    ) * SCALE
}

pub fn electromagnetism_simplified(
    mut query: Query<(&Transform, &mut Particle, &mut Movement)>,
    mut gizmos: Gizmos
){
    let mut combinations = query.iter_combinations_mut();

    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {

        let distance = distance(
            particle1.0.translation.x,
            particle1.0.translation.y,
            particle2.0.translation.x,
            particle2.0.translation.y
        );


        if distance/SCALE < 250.0 {
            //println!("distance: {}", distance/SCALE);
            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN
            );

            // for now not applying the charge value of the particles because it's always the absolut charge so 1 and therefore not needed
            let electrical_force = (COULOMBS_CONSTANT * crate::constants::ELEMENTARY_CHARGE.powf(2.0)) / distance.powf(2.0);

            //println!("electrical force: {}; distance: {}", electrical_force, distance*2.4/SCALE);

            //println!("distance:  {}", distance);

            let acc_dir1 = acceleration_direction_em(
                particle1.0.translation.x,
                particle1.0.translation.y,
                particle2.0.translation.x,
                particle2.0.translation.y,
                distance,
                particle1.1.charge,
                particle2.1.charge
            );

            //println!("dir one:  {:?}", acc_dir1);


            let acc_dir2 = acceleration_direction_em(
                particle2.0.translation.x,
                particle2.0.translation.y,
                particle1.0.translation.x,
                particle1.0.translation.y,
                distance,
                particle1.1.charge,
                particle2.1.charge
            );

            let curr_acceleration1x = electrical_force / particle1.1.mass * acc_dir1.x;
            let curr_acceleration1y = electrical_force / particle1.1.mass * acc_dir1.y;
            let curr_acceleration2x = electrical_force / particle2.1.mass * acc_dir2.x;
            let curr_acceleration2y = electrical_force / particle2.1.mass * acc_dir2.y;

            particle1.2.acceleration.x += curr_acceleration1x - particle1.2.prev_acceleration.x;
            particle1.2.acceleration.y += curr_acceleration1y - particle1.2.prev_acceleration.y;
            particle2.2.acceleration.x += curr_acceleration2x - particle2.2.prev_acceleration.x;
            particle2.2.acceleration.y += curr_acceleration2y - particle2.2.prev_acceleration.y;

            particle1.2.prev_acceleration.x = curr_acceleration1x;
            particle1.2.prev_acceleration.y = curr_acceleration1y;
            particle2.2.prev_acceleration.x = curr_acceleration2x;
            particle2.2.prev_acceleration.y = curr_acceleration2y;
        }
    }
}