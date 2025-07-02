use bevy::color::palettes::css::RED;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::{Movement, Particle};
use crate::constants::SCALE;

fn relationship_system (
    mut query: Query<(&Transform, &mut Particle, &mut Movement, Entity, Option<&Children>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) {

    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {
        let (&particle_transform1, mut particle_particle1, mut particle_movement1, particle_entity1, particle_children1) = (particle1.0, particle1.1, particle1.2, particle1.3, particle1.4);
        let (&particle_transform2, mut particle_particle2, mut particle_movement2, particle_entity2, particle_children2) = (particle2.0, particle2.1, particle2.2, particle2.3, particle2.4);

        let distance = particle_transform1.translation.distance(particle_transform2.translation) * SCALE;


        if particle_particle1.charge != 0.0 && particle_particle2.charge != 0.0 && distance / SCALE < 30.0 && particle_particle1.charge != particle_particle2.charge {
            particle_particle1.atomically_bound = true;
            particle_particle2.atomically_bound = true;

            let charge1 = particle_particle1.charge;
            let charge2 = particle_particle2.charge;

            let entity1 = particle_entity1;
            let entity2 = particle_entity2;

            let (child, parent, particle) = if charge2 == 1.0 {
                match particle_children1 {
                    Some(children) => (children, entity1, particle_particle1),
                    None => _,
                }
            } else {
                (Ok(particle_children2), entity2, particle_particle2)
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
    }
}