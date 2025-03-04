use bevy::prelude::{Query, Res, Time, Transform, With};
use crate::{Movement};

pub fn direction_system(mut q_transform: Query<&mut Movement, With<Movement>>
){
    for mut movement in q_transform.iter_mut() {
        movement.direction.x = movement.speed.x.signum();
        movement.direction.y = movement.speed.y.signum();
    }
}

pub fn move_system(mut q_transform: Query<(&mut Transform, &mut Movement), With<Movement>>){
    for (mut transform, mut movement) in q_transform.iter_mut() {
        movement.speed_limit(10.0);
        transform.translation.x += movement.speed.x;
        transform.translation.y += movement.speed.y;
    }
}

pub fn acceleration_system(
    mut query: Query<&mut Movement>,
    time: Res<Time>
){
    for mut movement in query.iter_mut() {

        movement.speed.x += movement.acceleration.x*time.delta_secs();
        movement.speed.y += movement.acceleration.y*time.delta_secs();

        println!("speed.x: {}, acc.x: {}, speed.y: {}, acc.y: {}", movement.speed.x, movement.acceleration.x, movement.speed.y, movement.acceleration.y);
    }
}




