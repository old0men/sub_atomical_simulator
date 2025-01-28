mod movement;
mod screen;
mod spawn;
mod physics;

use bevy::{color::palettes::basic::{BLUE, RED}, prelude::*};
pub const GREY: Srgba = Srgba::rgb(0.5, 0.5, 0.5);
use bevy::input::common_conditions::input_just_pressed;


struct Screen {
    width: f32,
    height: f32,
}
impl Screen {
    fn new(width: f32, height: f32) -> Self { Screen { width, height } }
}

#[derive(Component)]
struct Particle {
    mass: f32,
    charge: f32
}

#[derive(Component)]
struct Movement {
    speed: Vec2,
    acceleration: Vec2,
    prev_acceleration: Vec2,
    direction: Vec3
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (screen::spawn_camera, ))
        .add_systems(Update, (
            physics::electromagnetism,
            movement::direction_system,
            movement::acceleration_system,
            movement::move_system,
            screen::border_system,
            spawn_proton.pipe(spawn::spawn_particle)
                    .run_if(input_just_pressed(KeyCode::Digit1)),
            spawn_electron.pipe(spawn::spawn_particle)
                    .run_if(input_just_pressed(KeyCode::Digit2)),
            spawn_neutron.pipe(spawn::spawn_particle)
                    .run_if(input_just_pressed(KeyCode::Digit3)),
            spawn::spawn_particle_test.run_if(input_just_pressed(KeyCode::Digit4)),
            //clear_terminal
            ).chain(),
        )
        .run();
}

fn spawn_electron() -> Srgba {
    RED
}

fn spawn_proton() -> Srgba {
    BLUE
}

fn spawn_neutron() -> Srgba {
    GREY
}

fn clear_terminal(){
    print!("\x1B[2J\x1B[1;1H");
}