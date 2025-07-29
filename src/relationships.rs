use bevy::color::palettes::css::RED;
use bevy::math::Vec3;
use bevy::prelude::*;
use crate::{Particle, ParticleDifferentiatorParam};
use crate::constants::SCALE;

pub fn relationship_system (
    mut query: Query<(&Transform, &mut Particle, Entity, Option<&Children>)>,
    mut particle_diff: ParticleDifferentiatorParam,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) {

    let mut combinations = query.iter_combinations_mut();
    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {
        let (&particle_transform1, mut particle_particle1, particle_entity1, particle_children1) = (particle1.0, particle1.1, particle1.2, particle1.3);
        let (&particle_transform2, mut particle_particle2, particle_entity2, particle_children2) = (particle2.0, particle2.1, particle2.2, particle2.3);

        let distance = particle_transform1.translation.distance(particle_transform2.translation) * SCALE;

        if !particle_diff.is_neutron(particle_entity1) && !particle_diff.is_neutron(particle_entity2) && distance / SCALE < 30.0 && particle_particle1.charge != particle_particle2.charge {

            particle_particle1.atomically_bound = true;
            particle_particle2.atomically_bound = true;

            println!("BOUND-----------------------");
        }
    }

    for particle in query.iter_mut(){
        let (mut particle, entity, children) = (particle.1, particle.2, particle.3);

        println!("loop 2-------------");
        println!("electron count: {}, {}, {}", particle.electron_count, particle.atomically_bound, particle_diff.is_proton(entity));
        println!("--------------");

        if particle_diff.is_proton(entity) && particle.atomically_bound && particle.electron_count < 2.0 {

            match children {
                Some(children) => {
                    println!("got to here: ");
                    println!("{:?}", children);
                    println!("done");

                    for child in children {
                        commands.entity(*child).despawn_recursive();
                    }

                    commands.spawn((
                    Mesh2d(meshes.add(Annulus::new(particle.electron_count, particle.electron_count * 2.0))),
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
                    )).set_parent(entity);
                    particle.electron_count += 1.0;
                    particle.atomically_bound = false;
                    println!("UNBOUND------------------------------u")

                },
                None => {
                    println!("got to here first: ");
                    println!("{:?}", children);
                    println!("done");

                    commands.spawn((
                        Mesh2d(meshes.add(Annulus::new(particle.electron_count, particle.electron_count * 1.1))),
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
                    )).set_parent(entity);
                    particle.electron_count += 1.0;
                    particle.atomically_bound = false;
                    println!("UNBOUND------------------------------u")
                }
            }
        }

        if particle_diff.is_electron(entity) && particle.atomically_bound {
            commands.entity(entity).despawn_recursive();
        }
    }
}