use crate::{Movement, Particle, electromagnetism_calc as emc, strong_force_calc as sfc, ParticleDifferentiatorParam};
use bevy::prelude::*;
use bevy::color::palettes::css::{GREEN, ORANGE, RED};
use bevy::math::NormedVectorSpace;
use crate::constants::{SCALE};


pub fn acting_forces(
    mut query: Query<(&Transform, &mut Particle, &mut Movement, Entity)>,
    mut particle_diff: ParticleDifferentiatorParam,
    mut gizmos: Gizmos,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {

        let (&particle_transform1, mut particle_particle1, mut particle_movement1, particle_entity1) = (particle1.0, particle1.1, particle1.2, particle1.3);
        let (&particle_transform2, mut particle_particle2, mut particle_movement2, particle_entity2) = (particle2.0, particle2.1, particle2.2, particle2.3);

        let distance = particle_transform1.translation.distance(particle_transform2.translation) * SCALE;

        let direction_vector1 = (particle_transform2.translation - particle_transform1.translation) / (distance / SCALE);
        let direction_vector2 = (particle_transform1.translation - particle_transform2.translation) / (distance / SCALE);

        let velocity1 = particle_movement1.speed;
        let velocity2 = particle_movement2.speed;


        if distance / SCALE < 250.0 && !particle_diff.is_neutron(particle_entity1) && !particle_diff.is_neutron(particle_entity2) {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                GREEN,
            );


            let electrical_field1 = emc::electrical_field(
                particle_particle1.charge,
                direction_vector1,
                distance,
            );

            let electrical_field2 = emc::electrical_field(
                particle_particle2.charge,
                direction_vector2,
                distance,
            );

            if particle_particle2.charge == particle_particle1.charge {
                particle_particle1.total_electrical_field = -electrical_field1;
                particle_particle2.total_electrical_field = -electrical_field2;
            } else {
                particle_particle1.total_electrical_field = electrical_field1;
                particle_particle2.total_electrical_field = electrical_field2;
            }


            particle_particle1.total_magnetic_field = emc::magnetical_field(
                particle_particle1.charge,
                particle_movement1.speed.x,
                particle_movement1.speed.y,
                direction_vector1,
                distance,
            );

            particle_particle2.total_magnetic_field = emc::magnetical_field(
                particle_particle2.charge,
                particle_movement2.speed.x,
                particle_movement2.speed.y,
                direction_vector2,
                distance,
            );

            particle_particle1.total_lorentz_force = emc::loretz_force(
                particle_particle1.charge,
                particle_particle1.total_electrical_field,
                velocity1,
                particle_particle1.total_magnetic_field,
            );

            particle_particle2.total_lorentz_force = emc::loretz_force(
                particle_particle2.charge,
                particle_particle2.total_electrical_field,
                velocity2,
                particle_particle2.total_magnetic_field,
            );
        }


        if distance / SCALE < particle_transform1.scale.x + 1.0 && !particle_diff.is_electron(particle_entity1) && !particle_diff.is_electron(particle_entity2) {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                RED,
            );

            particle_particle1.total_strong_force = sfc::strong_force(direction_vector1, distance);
            particle_particle2.total_strong_force = sfc::strong_force(direction_vector2, distance);

        } else if distance / SCALE < particle_transform1.scale.x * 2.5 && !particle_diff.is_electron(particle_entity1) && !particle_diff.is_electron(particle_entity2) {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                ORANGE,
            );

            particle_particle1.total_strong_force = sfc::strong_force(-direction_vector1, distance);
            particle_particle2.total_strong_force = sfc::strong_force(-direction_vector2, distance);
        };

        let acceleration1 = (particle_particle1.total_lorentz_force + particle_particle1.total_strong_force) / particle_particle1.mass;
        let acceleration2 = (particle_particle2.total_lorentz_force + particle_particle2.total_strong_force) / particle_particle2.mass;

        particle_movement1.acceleration = acceleration1;
        particle_movement2.acceleration = acceleration2;
    }
}


