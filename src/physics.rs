use crate::{Movement, Particle, electromagnetism_calc as emc, strong_force_calc as sfc};
use bevy::prelude::*;
use bevy::color::palettes::css::{BLUE, GREEN, ORANGE, RED};
use bevy::math::NormedVectorSpace;
use crate::constants::{SCALE};


pub fn acting_forces(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Particle, &mut Movement, Entity, &Children)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {
        let distance = particle1.0.translation.distance(particle2.0.translation) * SCALE;
        println!("{distance}");

        let entity1 = particle1.3;
        let entity2 = particle2.3;

        let direction_vector1 = (particle2.0.translation - particle1.0.translation) / (distance / SCALE);
        let direction_vector2 = (particle1.0.translation - particle2.0.translation) / (distance / SCALE);

        let velocity1 = particle1.2.speed;
        let velocity2 = particle2.2.speed;
/*
        if particle1.1.charge != 0.0 && particle2.1.charge != 0.0 && distance / SCALE < 30.0 && particle1.1.charge != particle2.1.charge {
            particle1.1.atomically_bound = true;
            particle2.1.atomically_bound = true;


            let charge = (particle2.1.charge, particle1.1.charge);
            let mut parent: &(&Transform, Mut<Particle>, Mut<Movement>, Entity, &Children) = if charge.0 == 1.0 {
                &mut particle1
            } else {
                &mut particle2
            };

            if parent.4.len() <= 1.0 as usize {
                if charge.1 == -1.0 {
                    commands.entity(entity1).despawn_recursive();
                } else {
                    commands.entity(entity2).despawn_recursive();
                }

                commands.spawn((
                    Mesh2d(meshes.add(Annulus::new(parent.1.electron_count, parent.1.electron_count * 1.2))),
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
                )).set_parent(parent.3);

                parent.1.electron_count += 1.0;
            }
        }

 */

        if distance / SCALE < 250.0 && particle1.1.charge != 0.0 && particle2.1.charge != 0.0 {
            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN,
            );


            let electrical_field1 = emc::electrical_field(
                particle1.1.charge,
                direction_vector1,
                distance,
            );

            let electrical_field2 = emc::electrical_field(
                particle2.1.charge,
                direction_vector2,
                distance,
            );

            if particle2.1.charge == particle1.1.charge {
                particle1.1.total_electrical_field = -electrical_field1;
                particle2.1.total_electrical_field = -electrical_field2;
            } else {
                particle1.1.total_electrical_field = electrical_field1;
                particle2.1.total_electrical_field = electrical_field2;
            }


            particle1.1.total_magnetic_field = emc::magnetical_field(
                particle1.1.charge,
                particle1.2.speed.x,
                particle1.2.speed.y,
                direction_vector1,
                distance,
            );

            particle2.1.total_magnetic_field = emc::magnetical_field(
                particle2.1.charge,
                particle2.2.speed.x,
                particle2.2.speed.y,
                direction_vector2,
                distance,
            );

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


        if distance / SCALE < particle1.0.scale.x + 1.0 && particle1.1.charge != -1.0 && particle2.1.charge != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                RED,
            );

            particle1.1.total_strong_force = sfc::strong_force(direction_vector1, distance);
            particle2.1.total_strong_force = sfc::strong_force(direction_vector2, distance);
        } else if distance / SCALE < particle1.0.scale.x * 2.5 && particle1.1.charge != -1.0 && particle2.1.charge != -1.0 {
            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                ORANGE,
            );

            particle1.1.total_strong_force = sfc::strong_force(-direction_vector1, distance);
            particle2.1.total_strong_force = sfc::strong_force(-direction_vector2, distance);
        };


        let acceleration1 = (particle1.1.total_lorentz_force / particle1.1.mass + particle1.1.total_strong_force) / particle1.1.mass;
        let acceleration2 = (particle2.1.total_lorentz_force / particle2.1.mass + particle2.1.total_strong_force) / particle2.1.mass;

        particle1.2.acceleration = acceleration1;
        particle2.2.acceleration = acceleration2;
    }
}


