use bevy::prelude::{Camera2d, Commands, Query, Single, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::{Movement, Screen};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub fn check_screen(window: &Window) -> Screen {
    let width = window.resolution.width();
    let height = window.resolution.height();
    Screen::new(width, height)
}

pub fn border_system(
    q_windows: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut Movement), With<Movement>>
){
    let screen = check_screen(*q_windows);
    for (transform, mut movement ) in query.iter_mut() {
        if transform.translation.x >= screen.width/2.0 || transform.translation.x <= -screen.width/2.0 {
            //println!("pos:{:?}, width:{:?}", transform.translation.x, screen.width);
            movement.speed.x *= -1.0
        }
        else if transform.translation.y >= screen.height/2.0 || transform.translation.y <= -screen.height/2.0 {
            //println!("pos:{:?}, height:{:?}", transform.translation.y, screen.height);
            movement.speed.y *= -1.0
        }
    }
}