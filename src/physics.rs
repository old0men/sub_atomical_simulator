use crate::{Movement, Particle, electromagnetism_calc as emc, strong_force_calc as sfc, Proton, Neutron, Electron};
use bevy::prelude::*;
use bevy::color::palettes::css::{BLUE, GREEN, ORANGE, RED};
use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::SystemParam;
use bevy::math::NormedVectorSpace;
use crate::constants::{SCALE};
use crate::spawn::ParticleType;

#[derive(SystemParam)]
struct ParticleDifferentiatorParam<'w,'s> {
    particles: Query<'w, 's, (Has<Proton>, Has<Neutron>, Has<Electron>)>,

}

impl ParticleDifferentiatorParam<'_,'_> {
    //fn get_type() -> ParticleType

    fn is_proton(&self, entity: Entity) -> bool {
        if let Some(res) = self.get_entity(entity) {
            res.0
        } else {
            // Handle Entity not found
            false
        }
    }

    fn get_entity(&self, entity: Entity) -> Option<(bool, bool, bool)> {
        match self.particles.get(entity) {
            Ok(res) => {
                Some(res)
            }
            Err(_) => {
                None
            }
        }
    }
}

pub fn acting_forces(
    mut query: Query<(&Transform, &mut Particle, &mut Movement, Entity, Option<&Children>)>,
    mut particle_differentiator_param: ParticleDifferentiatorParam,
    mut gizmos: Gizmos,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {

        let (&particle_transform1, mut particle_particle1, mut particle_movement1, particle_entity1, particle_children1) = (particle1.0, particle1.1, particle1.2, particle1.3, particle1.4);
        let (&particle_transform2, mut particle_particle2, mut particle_movement2, particle_entity2, particle_children2) = (particle2.0, particle2.1, particle2.2, particle2.3, particle2.4);

        let distance = particle_transform1.translation.distance(particle_transform2.translation) * SCALE;


        if particle_differentiator_param.is_proton(entity1) {
            // PROTON
        }

        let direction_vector1 = (particle_transform2.translation - particle_transform1.translation) / (distance / SCALE);
        let direction_vector2 = (particle_transform1.translation - particle_transform2.translation) / (distance / SCALE);

        let velocity1 = particle_movement1.speed;
        let velocity2 = particle_movement2.speed;


        if distance / SCALE < 250.0 && particle_particle1.charge != 0.0 && particle_particle2.charge != 0.0 {
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


        if distance / SCALE < particle_transform1.scale.x + 1.0 && particle_particle1.charge != -1.0 && particle_particle2.charge != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                RED,
            );

            particle_particle1.total_strong_force = sfc::strong_force(direction_vector1, distance);
            particle_particle2.total_strong_force = sfc::strong_force(direction_vector2, distance);
        } else if distance / SCALE < particle_transform1.scale.x * 2.5 && particle_particle1.charge != -1.0 && particle_particle2.charge != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                ORANGE,
            );

            particle_particle1.total_strong_force = sfc::strong_force(-direction_vector1, distance);
            particle_particle2.total_strong_force = sfc::strong_force(-direction_vector2, distance);
        };


        let acceleration1 = (particle_particle1.total_lorentz_force / particle_particle1.mass + particle_particle1.total_strong_force) / particle_particle1.mass;
        let acceleration2 = (particle_particle2.total_lorentz_force / particle_particle2.mass + particle_particle2.total_strong_force) / particle_particle2.mass;

        particle_movement1.acceleration = acceleration1;
        particle_movement2.acceleration = acceleration2;
    }
}


