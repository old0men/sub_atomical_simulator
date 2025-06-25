use bevy::color::palettes::css::GREEN;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Gizmos, Query, Transform};
use crate::{Movement, Particle, constants as cns};

fn acceleration_direction_gravity(from_x1: f32, from_y1: f32, to_x1: f32, to_y1: f32, distance: f32) -> Vec3 {
    let result = Vec3::new((to_x1-from_x1)/distance,
                           (to_y1-from_y1)/distance,
                           0.0);
    result
}

fn distance(
    from_x1: f32, from_y1: f32, to_x1: f32, to_y1: f32
) -> f32 {
    (   (  (to_x1 - from_x1).powf(2.0)  +  (to_y1 - from_y1).powf(2.0) ).sqrt()    ) * cns::SCALE
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

        if distance/cns::SCALE < 250.0
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