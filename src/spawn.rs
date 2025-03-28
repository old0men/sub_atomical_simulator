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

        let parent = commands.spawn((
            Name::new(format!("Particle {:?}", charge)),
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::from(color))),
            Transform::from_xyz(position.x - screen.width, (position.y - screen.height) * -1.0, 0.0)
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
                total_electrical_field: Vec3::ZERO,
                total_magnetic_field: Vec3::ZERO,
                total_strong_force: Vec3::ZERO,
                total_lorentz_force: Vec3::ZERO,
            }
        ))/*.with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(GREEN))),
                Transform::from_xyz(0.75, 0.75, 0.0)
                    .with_scale(Vec3::splat(0.5)),
                )
            );
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(Color::from(RED))),
                Transform::from_xyz(1.25, 0.75, 0.0)
                    .with_scale(Vec3::splat(0.5)),
            )
            );
        })*/;
    }
}

pub fn spawn_particle_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {


    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(16.0 * 1.75)),
        GlobalTransform::default(),
        Movement {
            speed: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            prev_acceleration: Vec3::ZERO,
            direction: Vec3::new(1.0, 1.0, 0.0),
            acceleration_counter: 0.0
        },
        Particle {
            mass: 1000000000000000000000000000000000.0,
            charge: 1.0,
            total_electrical_field: Vec3::ZERO,
            total_magnetic_field: Vec3::ZERO,
            total_strong_force: Vec3::new(10000000000000.0, 100000000000000000000.0, 0.0),
            total_lorentz_force: Vec3::ZERO,
        }
    ));


}







