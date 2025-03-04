use crate::{Movement, Particle, constants as cns, electromagnetism_calc as emc, strong_force_calc as sfc};
use bevy::prelude::*;
use bevy::color::palettes::css::{GREEN, ORANGE, RED};
use crate::constants::{COULOMBS_CONSTANT, SCALE};

fn acceleration_direction_gravity(from_x1: f32, from_y1: f32, to_x1: f32, to_y1: f32, distance: f32) -> Vec3 {
    let result = Vec3::new((to_x1-from_x1)/distance,
                           (to_y1-from_y1)/distance,
                           0.0);
    result
}

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

#[warn(dead_code)]
pub fn gravity(
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

        if distance/SCALE < 250.0
        {
            //println!("{distance}");

            let direction_particle1 = acceleration_direction_gravity(
                                                        particle1.0.translation.x,
                                                        particle1.0.translation.y,
                                                        particle2.0.translation.x,
                                                        particle2.0.translation.y,
                                                        distance
            );

            let direction_particle2 = acceleration_direction_gravity(
                                                        particle2.0.translation.x,
                                                        particle2.0.translation.y,
                                                        particle1.0.translation.x,
                                                        particle1.0.translation.y,
                                                        distance
            );

            let current_acceleration_change1 = Vec2::new(cns::FORCE_GRAVITY*((particle2.1.mass/(distance*0.000003268).powf(2.0))*direction_particle1.x),
                                                                cns::FORCE_GRAVITY*((particle2.1.mass/(distance*0.00003268).powf(2.0))*direction_particle1.y));

            let current_acceleration_change2 = Vec2::new(cns::FORCE_GRAVITY*((particle1.1.mass/(distance*0.000003268).powf(2.0))*direction_particle2.x),
                                                                cns::FORCE_GRAVITY*((particle1.1.mass/(distance*0.00003268).powf(2.0))*direction_particle2.y));


            particle1.2.acceleration.x = current_acceleration_change1.x;
            particle1.2.acceleration.y = current_acceleration_change1.y;
            particle2.2.acceleration.x = current_acceleration_change2.x;
            particle2.2.acceleration.y = current_acceleration_change2.y;


            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN
            );
        }
    }
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
            let electrical_force = (COULOMBS_CONSTANT * cns::ELEMENTARY_CHARGE.powf(2.0)) / distance.powf(2.0);

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

pub fn acting_forces (
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

        let direction_vector1 = Vec3::new((particle2.0.translation.x-particle1.0.translation.x)/(distance/SCALE),
                                          (particle2.0.translation.y-particle1.0.translation.y)/(distance/SCALE),
                                          0.0,);
        let direction_vector2 = Vec3::new((particle1.0.translation.x-particle2.0.translation.x)/(distance/SCALE),
                                          (particle1.0.translation.y-particle2.0.translation.y)/(distance/SCALE),
                                          0.0);

        let velocity1 = Vec3::new(particle1.2.speed.x, particle1.2.speed.y, 0.0);
        let velocity2 = Vec3::new(particle2.2.speed.x, particle2.2.speed.y, 0.0);

        if distance/SCALE < 250.0 && particle1.1.charge != 0.0 && particle2.1.charge != 0.0{

            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN
            );

            /*
            println!("1x: {}, 1y: {}, 2x: {}, 2y: {}", particle1.0.translation.x, particle1.0.translation.y, particle2.0.translation.x, particle2.0.translation.y);
            println!("distance in femtometers: {}", distance/SCALE);
             */

            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN
            );


            let electrical_field1 = emc::electrical_field(
                particle1.1.charge,
                direction_vector1,
                distance
            );

            let electrical_field2 = emc::electrical_field(
                particle2.1.charge,
                direction_vector2,
                distance
            );

            if particle2.1.charge == particle1.1.charge {
                particle1.1.total_electrical_field = -electrical_field1;
                particle2.1.total_electrical_field = -electrical_field2;
            } else {
                particle1.1.total_electrical_field = electrical_field1;
                particle2.1.total_electrical_field = electrical_field2;
            }


            let magnetic_field1 = emc::magnetical_field(
                particle1.1.charge,
                particle1.2.speed.x,
                particle1.2.speed.y,
                direction_vector1,
                distance,
            );

            let magnetic_field2 = emc::magnetical_field(
                particle2.1.charge,
                particle2.2.speed.x,
                particle2.2.speed.y,
                direction_vector2,
                distance,
            );


            particle1.1.total_magnetic_field = magnetic_field1;
            particle2.1.total_magnetic_field = magnetic_field2;


            particle1.1.total_lorentz_force = emc::loretz_force(
                particle1.1.charge,
                particle1.1.total_electrical_field,
                velocity1,
                particle1.1.total_magnetic_field,
            );

            particle2.1.total_lorentz_force = emc::loretz_force(
                particle2.1.charge,
                particle2.1.total_electrical_field,
                velocity2,
                particle2.1.total_magnetic_field,
            );
        }



        if distance/SCALE < particle1.0.scale.x+1.0 && particle1.1.charge != -1.0 && particle2.1.charge != -1.0{

            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                RED
            );

            particle1.1.total_strong_force = sfc::strong_force(direction_vector1, distance);
            particle2.1.total_strong_force = sfc::strong_force(direction_vector2, distance);

        } else if distance/SCALE < particle1.0.scale.x*2.02 && particle1.1.charge != -1.0 && particle2.1.charge != -1.0{

            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                ORANGE
            );

            particle1.1.total_strong_force = sfc::strong_force(-direction_vector1, distance);
            particle2.1.total_strong_force = sfc::strong_force(-direction_vector2, distance);
        };

        let acceleration1 = (particle1.1.total_lorentz_force + particle1.1.total_strong_force)/particle1.1.mass;
        let acceleration2 = (particle2.1.total_lorentz_force + particle2.1.total_strong_force)/particle2.1.mass;

        particle1.2.acceleration = acceleration1;
        particle2.2.acceleration = acceleration2;


    }
}

pub fn moc_electromagnetism(
    mut query: Query<(&Transform, &Particle, &mut Movement)>
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
            let electrical_field = (COULOMBS_CONSTANT*cns::ELEMENTARY_CHARGE.powf(2.0))/distance.powf(2.0);

            let direction1 = Vec3::new((particle2.0.translation.x-particle1.0.translation.x)/(distance/SCALE),
                                           (particle2.0.translation.y-particle1.0.translation.y)/(distance/SCALE),
                                           0.0,);
            let direction2 = Vec3::new((particle1.0.translation.x-particle2.0.translation.x)/(distance/SCALE),
                                              (particle1.0.translation.y-particle2.0.translation.y)/(distance/SCALE),
                                              0.0);

            particle1.2.acceleration = -1.0*particle1.1.charge*particle2.1.charge*direction1*electrical_field;
            particle2.2.acceleration = -1.0*particle2.1.charge*particle1.1.charge*direction2*electrical_field;
        }
    }
}



