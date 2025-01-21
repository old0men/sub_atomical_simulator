use bevy::prelude::*;
use bevy::{color::palettes::css::{GREEN}};
use crate::{Particle, Movement};

#[warn(dead_code)]
pub fn gravity(
    mut query: Query<(&Transform, &mut Particle, &mut Movement), With<Particle>>,
    mut gizmos: Gizmos
){
    pub const  FORCE_GRAVITY: f32 = 6.67430e-11f32;

    let mut combinations = query.iter_combinations_mut();


    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {


        let distance = ((particle2.0.translation.x - particle1.0.translation.x).powf(2.0)+(particle2.0.translation.y - particle1.0.translation.y).powf(2.0)).sqrt();


        if distance < 250.0
        {
            //println!("{distance}");

            let direction_particle1 = Vec3::new((particle2.0.translation.x-particle1.0.translation.x)/distance,
                                                   (particle2.0.translation.y-particle1.0.translation.y)/distance,
                                                   0.0);

            let direction_particle2 = Vec3::new((particle1.0.translation.x-particle2.0.translation.x)/distance,
                                                (particle1.0.translation.y-particle2.0.translation.y)/distance,
                                                0.0);

            let current_acceleration_change1 = Vec2::new(FORCE_GRAVITY*((particle2.1.mass/(distance*0.000003268).powf(2.0))*direction_particle1.x), FORCE_GRAVITY*((particle2.1.mass/(distance*0.00003268).powf(2.0))*direction_particle1.y));
            let current_acceleration_change2 = Vec2::new(FORCE_GRAVITY*((particle1.1.mass/(distance*0.000003268).powf(2.0))*direction_particle2.x), FORCE_GRAVITY*((particle1.1.mass/(distance*0.00003268).powf(2.0))*direction_particle2.y));


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