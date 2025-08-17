use crate::{Movement, Particle, electromagnetism_calc as emc, strong_force_calc as sfc};
use bevy::prelude::*;
use bevy::color::palettes::css::{BLUE, GREEN, ORANGE, RED};
use bevy::math::NormedVectorSpace;
use crate::constants::{SCALE};


pub fn acting_forces(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Particle, &mut Movement, Entity, Option<&Children>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {

        let (&particle_transform1, mut particle1, mut particle_movement1, particle_entity1, particle_children1) = (particle1.0, particle1.1, particle1.2, particle1.3, particle1.4);
        let (&particle_transform2, mut particle2, mut particle_movement2, particle_entity2, particle_children2) = (particle2.0, particle2.1, particle2.2, particle2.3, particle2.4);

        let distance = particle_transform1.translation.distance(particle_transform2.translation) * SCALE;

        let entity1 = particle_entity1;
        let entity2 = particle_entity2;

        let direction_vector1 = (particle_transform2.translation - particle_transform1.translation) / (distance / SCALE);
        let direction_vector2 = (particle_transform1.translation - particle_transform2.translation) / (distance / SCALE);

        let velocity1 = particle_movement1.speed;
        let velocity2 = particle_movement2.speed;

        let charge1 = particle1.charge;
        let charge2 = particle2.charge;



        if distance / SCALE < 250.0 && charge1 != 0.0 && charge2 != 0.0 {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                GREEN,
            );


            let electrical_field1 = emc::electrical_field(
                charge1,
                direction_vector1,
                distance,
            );

            let electrical_field2 = emc::electrical_field(
                charge2,
                direction_vector2,
                distance,
            );

            if charge2 == charge1 {
                particle1.total_electrical_field = -electrical_field1;
                particle2.total_electrical_field = -electrical_field2;
            } else {
                particle1.total_electrical_field = electrical_field1;
                particle2.total_electrical_field = electrical_field2;
            }


            particle1.total_magnetic_field = emc::magnetical_field(
                charge1,
                velocity1.x,
                velocity1.y,
                direction_vector1,
                distance,
            );

            particle2.total_magnetic_field = emc::magnetical_field(
                charge2,
                velocity2.x,
                velocity2.y,
                direction_vector2,
                distance,
            );

            particle1.total_lorentz_force = emc::loretz_force(
                charge1,
                particle1.total_electrical_field,
                velocity1,
                particle1.total_magnetic_field,
                direction_vector1
            );

            particle2.total_lorentz_force = emc::loretz_force(
                charge2,
                particle2.total_electrical_field,
                velocity2,
                particle2.total_magnetic_field,
                direction_vector2
            );
        }


        if distance / SCALE < particle_transform1.scale.x + 1.0 && charge1 != -1.0 && charge2 != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                RED,
            );

            particle1.total_strong_force = sfc::strong_force(direction_vector1, distance);
            particle2.total_strong_force = sfc::strong_force(direction_vector2, distance);
            
        } else if distance / SCALE < particle_transform1.scale.x * 2.5 && charge1 != -1.0 && charge2 != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle_transform2.translation.x, particle_transform2.translation.y),
                Vec2::new(particle_transform1.translation.x, particle_transform1.translation.y),
                ORANGE,
            );

            particle1.total_strong_force = sfc::strong_force(-direction_vector1, distance);
            particle2.total_strong_force = sfc::strong_force(-direction_vector2, distance);
        };


        let acceleration1 = (particle1.total_lorentz_force / particle1.mass + particle1.total_strong_force) / particle1.mass;
        let acceleration2 = (particle2.total_lorentz_force / particle2.mass + particle2.total_strong_force) / particle2.mass;

        particle_movement1.acceleration = acceleration1;
        particle_movement2.acceleration = acceleration2;

        particle1.total_lorentz_force = Vec3::ZERO;
        particle1.total_strong_force = Vec3::ZERO;

        particle2.total_lorentz_force = Vec3::ZERO;
        particle2.total_strong_force = Vec3::ZERO;
    }
}



/*

        ##### old forced system to have electron become rings around protons. This system isn't needed anymore ####

        if particle1.charge != 0.0 && particle2.charge != 0.0 && distance / SCALE < 30.0 && particle1.charge != particle2.charge {
            particle1.atomically_bound = true;
            particle2.atomically_bound = true;


            let (child, parent, particle) = if charge2 == 1.0 {
                match particle_children1 {
                    Some(children) => (children, entity1, particle1),
                    None => _,
                }
            } else {
                (Ok(particle_children2), entity2, particle2)
            };

            if child.len() <= 1.0 as usize {
                if charge1 == -1.0 {
                    commands.entity(entity1).despawn_recursive();
                } else {
                    commands.entity(entity2).despawn_recursive();
                }

                commands.spawn((
                    Mesh2d(meshes.add(Annulus::new(particle.electron_count, particle.electron_count * 1.2))),
                    MeshMaterial2d(materials.add(Color::from(RED))),
                    GlobalTransform::default(),
                    Particle {
                        mass: 100.0,
                        charge: -1.0,
                        atomically_bound: false,
                        nuclear_bound: false,
                        total_electrical_field: Vec3::ZERO,
                        total_magnetic_field: Vec3::ZERO,
                        total_strong_force: Vec3::ZERO,
                        total_lorentz_force: Vec3::ZERO,
                        electron_count: 0.0,
                    }
                )).set_parent(parent);

                particle.electron_count += 1.0;
            }
        }

 */


