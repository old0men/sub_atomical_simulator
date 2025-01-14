use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Time, Transform, Vec3, With};
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

pub fn direction_check(to: Vec3, from: Vec3) -> Vec3 {
    let mut direction_x =0.0;
    let mut direction_y =0.0;
    if from.x - to.x < 0.0 {
        direction_x = -1.0
    } else {
        direction_x = 1.0
    }
    if from.y - to.y < 0.0 {
        direction_y = -1.0
    } else {
        direction_y = 1.0
    }
    Vec3::new(direction_x, direction_y, 0.0)}

pub fn move_system(mut q_transform: Query<(&mut Transform, &Movement), With<Movement>>){
    for (mut transform, movement) in q_transform.iter_mut() {
        if movement.speed.x < 50.0 || movement.speed.y < 50.0{
            transform.translation.x += movement.speed.x;
            transform.translation.y += movement.speed.y;
        } else if movement.speed.x < -50.0 || movement.speed.y < -50.0{
            transform.translation.x += movement.speed.x;
            transform.translation.y += movement.speed.y;
        } else if movement.speed.x.signum() == 1.0 || movement.speed.y.signum() == 1.0 {
            transform.translation.x += 50.0;
            transform.translation.y += 50.0;
        } else {
            transform.translation.x -= 50.0;
            transform.translation.y -= 50.0;
        }
        println!("x: {}, y: {}", transform.translation.x, transform.translation.y);
    }
}

pub fn acceleration_system(
    mut query: Query<&mut Movement>,
    buttons: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    for mut movement in query.iter_mut() {

        if movement.direction.x > 0.0 && movement.acceleration.x * time.delta_secs() > 0.0 {
            movement.speed.x += movement.acceleration.x * time.delta_secs()
        }

        if movement.direction.x < 0.0 && movement.acceleration.x * time.delta_secs() < 0.0 {
            movement.speed.x += -movement.acceleration.x * time.delta_secs()
        }

        if movement.direction.y > 0.0 && movement.acceleration.y * time.delta_secs() > 0.0 {
            movement.speed.y += movement.acceleration.y * time.delta_secs()
        }

        if movement.direction.y < 0.0 && movement.acceleration.y * time.delta_secs() < 0.0 {
            movement.speed.y += -movement.acceleration.y * time.delta_secs()
        }

        if buttons.just_pressed(KeyCode::ArrowUp) {
            movement.acceleration.y += 0.1;
            movement.acceleration.x += 0.1;
        }
        if buttons.just_pressed(KeyCode::ArrowDown) {
            movement.acceleration.y -= 0.1;
            movement.acceleration.x -= 0.1;
        }/*
        if movement.acceleration.x < 0.1 && movement.acceleration.x > -0.1 || movement.acceleration.y < 0.1 && movement.acceleration.y > -0.1{
            movement.acceleration.x = 0.0;
            movement.acceleration.y = 0.0;
        }
        */

        //println!("speed.x: {}, acc.x: {}, speed.y: {}, acc.y: {}", movement.speed.x, movement.acceleration.x, movement.speed.y, movement.acceleration.y);
    }
}
