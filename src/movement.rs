use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Time, Transform, With};
use crate::Movement;

pub fn direction_system(mut q_transform: Query<&mut Movement, With<Movement>>
){
    for mut movement in q_transform.iter_mut() {
        // im checking the speed of the particle because weather the object is moving in a positive or negative direction is important
        movement.direction.y = movement.speed.y.signum();
        movement.direction.x = movement.speed.x.signum();
        println!("X: {:?}, Y: {:?} acc: {:?}", movement.direction.x, movement.direction.y, movement.acceleration.x);
    }
}

pub fn move_system(mut q_transform: Query<(&mut Transform, &Movement), With<Movement>>){
    for (mut transform, movement) in q_transform.iter_mut() {
        if movement.speed.x < 10.0 && movement.speed.x > -10.0{
            transform.translation.x += movement.speed.x;
        } else {
            transform.translation.x += 10.0*movement.direction.x;
        }
        if movement.speed.y < 10.0 && movement.speed.y > -10.0{
            transform.translation.y += movement.speed.y;
        } else {
            transform.translation.y += 10.0*movement.direction.y;
        }

        println!("x: {}, y: {}", transform.translation.x, transform.translation.y);
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
