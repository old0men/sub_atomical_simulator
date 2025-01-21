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
        let mut mass:f32 = 0.0;
        if color == RED {mass=1.0}
        else if color == BLUE {mass=1836.0}
        else if color == GREY {mass=1836.0}

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::from(color))),
            Transform::from_xyz(position.x - (screen.width / 2.0), (position.y - (screen.height / 2.0)) * -1.0, 0.0)
                .with_scale(Vec3::splat(16.)),
            GlobalTransform::default(),
            Movement {
                speed: Vec2::new(0.0, 0.0),
                acceleration: Vec2::ZERO,
                direction: Vec3::new(1.0, 1.0, 0.0)
            },
            Particle{ mass }
            )
        );
    }
}
