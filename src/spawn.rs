use bevy::{color::palettes::basic::{BLUE, RED}, prelude::*};
use bevy::window::PrimaryWindow;
use crate::{screen, Electron, Movement, Neutron, Particle, Proton, GREY};

pub struct SpawnParticle {
    my_type: ParticleType,
}

impl SpawnParticle {
    pub fn new(my_type: ParticleType) -> Self { SpawnParticle { my_type } }
}

pub enum ParticleType {
    PROTON,
    NEUTRON,
    ELECTRON,
}

impl Command for SpawnParticle {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
            world.resource_scope(|world, mut materials: Mut<Assets<ColorMaterial>>| {
                let q_windows = world.query_filtered::<&Window, With<PrimaryWindow>>();

                if let Some(position) = q_windows.cursor_position() {
                    let screen = screen::check_screen(*q_windows);
                    let mut mass: f32 = 0.0;
                    let mut charge: f32 = 0.0;
                    let mut diameter: f32 = 16.0;

                    let entity = world.spawn((
                        Name::new(format!("Particle {:?}", charge)),
                        Mesh2d(meshes.add(Circle::default())),
                        GlobalTransform::default(),
                        Movement {
                            speed: Vec3::ZERO,
                            acceleration: Vec3::ZERO,
                            prev_acceleration: Vec3::ZERO,
                            direction: Vec3::new(1.0, 1.0, 0.0),
                            acceleration_counter: 0.0,
                        },
                    )).id();

                    let color = match self.my_type {
                        ParticleType::PROTON => {
                            mass = 1836.0;
                            charge = 1.0;
                            diameter *= 1.75;
                            world.entity_mut(entity).insert(Proton);
                            BLUE
                        }
                        ParticleType::NEUTRON => {
                            mass = 1836.0;
                            diameter *= 1.75;
                            world.entity_mut(entity).insert(Neutron);
                            GREY
                        }
                        ParticleType::ELECTRON => {
                            mass = 100.0;
                            charge = -1.0;
                            world.entity_mut(entity).insert(Electron);
                            RED
                        }
                    };

                    world.entity_mut(entity).insert((
                        MeshMaterial2d(materials.add(Color::from(color))),
                        Transform::from_xyz(position.x - screen.width, screen.height - position.y, 0.0)
                            .with_scale(Vec3::splat(diameter)),
                        Particle {
                            mass,
                            charge,
                            atomically_bound: false,
                            nuclear_bound: false,
                            total_electrical_field: Vec3::ZERO,
                            total_magnetic_field: Vec3::ZERO,
                            total_strong_force: Vec3::ZERO,
                            total_lorentz_force: Vec3::ZERO,
                            electron_count: 0.0,
                        }));
                }
            });
        });
    }
}

pub fn spawn_particle(
    In(color): In<Srgba>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.cursor_position() {
        let screen = screen::check_screen(*q_windows);
        let mut mass: f32 = 0.0;
        let mut charge: f32 = 0.0;
        let mut diameter: f32 = 16.0;
        if color == RED {
            mass = 100.0;
            charge = -1.0;
        } else if color == BLUE {
            mass = 1836.0;
            charge = 1.0;
            diameter *= 1.75
        } else if color == GREY {
            mass = 1836.0;
            diameter *= 1.75
        }

        commands.spawn((
            Name::new(format!("Particle {:?}", charge)),
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::from(color))),
            Transform::from_xyz(position.x - screen.width, screen.height - position.y, 0.0)
                .with_scale(Vec3::splat(diameter)),
            GlobalTransform::default(),
            Movement {
                speed: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                prev_acceleration: Vec3::ZERO,
                direction: Vec3::new(1.0, 1.0, 0.0),
                acceleration_counter: 0.0,
            },
            Particle {
                mass,
                charge,
                atomically_bound: false,
                nuclear_bound: false,
                total_electrical_field: Vec3::ZERO,
                total_magnetic_field: Vec3::ZERO,
                total_strong_force: Vec3::ZERO,
                total_lorentz_force: Vec3::ZERO,
                electron_count: 0.0,
            }
        ));
    }
}






