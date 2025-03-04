use bevy::prelude::{Camera2d, Commands, Query, Single, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::{Movement, Screen};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub fn check_screen(window: &Window) -> Screen {
    let width = window.resolution.width()/2.0;
    let height = window.resolution.height()/2.0;
    Screen::new(width, height)
}


pub fn border_system(
    q_windows: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Movement>>
){
    let screen = check_screen(*q_windows);
    for (mut transform, mut movement ) in query.iter_mut() {
        if transform.translation.x.abs() >= screen.width {
            //println!("pos:{:?}, width:{:?}", transform.translation.x, screen.width);
            if transform.translation.x.abs() >= screen.width + 5.0{
                transform.translation.x = screen.width.copysign(transform.translation.x) + 40.0_f32.copysign(-transform.translation.x)
            }
            movement.speed.x *= -1.0
        }
        if transform.translation.y.abs() >= screen.height {
            //println!("pos:{:?}, height:{:?}", transform.translation.y, screen.height);
            if transform.translation.y.abs() >= screen.height + 5.0{
                transform.translation.y = screen.height.copysign(transform.translation.y) + 40.0_f32.copysign(-transform.translation.y)
            }
            movement.speed.y *= -1.0
        }
    }
}