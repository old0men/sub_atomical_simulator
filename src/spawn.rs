use bevy::{color::palettes::basic::{BLUE, RED}, prelude::*};
use bevy::window::PrimaryWindow;
use crate::{screen, Movement, Particle, GREY};

pub fn spawn_particle(
    In(color): In<Srgba>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Transform, &mut Movement)>,
    q_windows: Single<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.cursor_position() {
        let screen = screen::check_screen(*q_windows);
        let mut mass: f32 = 0.0;
        let mut charge: f32 = 0.0;
        let mut diameter: f32 = 16.0;
        if color == RED {
            mass = 1.0;
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
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::from(color))),
            Transform::from_xyz(position.x - screen.width, (position.y - screen.height) * -1.0, 0.0)
                .with_scale(Vec3::splat(diameter)),
            GlobalTransform::default(),
            Movement {
                speed: Vec2::new(0.0, 0.0),
                acceleration: Vec2::ZERO,
                prev_acceleration: Vec2::ZERO,
                direction: Vec3::new(1.0, 1.0, 0.0),
            },
            Particle {
                mass,
                charge,
            }
        )).with_children(|parent| {
            parent.spawn((
                Text2d::new("0.000005698"),
                TextFont {
                    font: Default::default(),
                    font_size: 50.0,

                    font_smoothing: Default::default(),
                },
                TextLayout::new_with_justify(JustifyText::Justified),
                Transform::from_translation(Vec3::new(1.0, 1.0, 0.0)),
                TextColor(Color::WHITE),
            )
            );
        });

    }
}

pub fn spawn_particle_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut diameter: f32 = 16.0;


    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(76.0, 35.0, 0.0)
            .with_scale(Vec3::splat(diameter * 1.75)),
        GlobalTransform::default(),
        Movement {
            speed: Vec2::new(0.0, 0.0),
            acceleration: Vec2::ZERO,
            prev_acceleration: Vec2::ZERO,
            direction: Vec3::new(1.0, 1.0, 0.0),
        },
        Particle {
            mass: 1836.0,
            charge: 1.0,
        }
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(21.0, -11.0, 0.0)
            .with_scale(Vec3::splat(diameter * 1.75)),
        GlobalTransform::default(),
        Movement {
            speed: Vec2::new(0.0, 0.0),
            acceleration: Vec2::ZERO,
            prev_acceleration: Vec2::ZERO,
            direction: Vec3::new(1.0, 1.0, 0.0),
        },
        Particle {
            mass: 1836.0,
            charge: 1.0,
        }
    ));
}







