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
        if movement.speed.x < 4.5 && movement.speed.x > -4.5{
            transform.translation.x += movement.speed.x;
            println!("movement speed x: {}, direction: {}", movement.speed.x, movement.direction.x);
        } else {
            movement.speed.x = 4.5*movement.direction.x;
            transform.translation.x += movement.speed.x;
            println!("movement speed x: {}, direction: {}", movement.speed.x, movement.direction.x);
        }
        if movement.speed.y < 4.5 && movement.speed.y > -4.5{
            transform.translation.y += movement.speed.y;
            println!("movement speed y: {}, direction: {}", movement.speed.y, movement.direction.y);
        } else {
            movement.speed.y = 4.5*movement.direction.y;
            transform.translation.y += movement.speed.y;
            println!("movement speed y: {}, direction: {}", movement.speed.y, movement.direction.y);
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




