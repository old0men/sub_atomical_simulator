use bevy::{color::palettes::basic::{BLUE, RED}, prelude::*};
use bevy::window::PrimaryWindow;
use crate::{screen, Movement, Particle, GREY};

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
            Transform::from_xyz(position.x - screen.width, screen.height - position.y , 0.0)
                .with_scale(Vec3::splat(diameter)),
            GlobalTransform::default(),
            Movement {
                speed: Vec3::ZERO,
                acceleration: Vec3::ZERO,
                prev_acceleration: Vec3::ZERO,
                direction: Vec3::new(1.0, 1.0, 0.0),
                acceleration_counter: 0.0
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






