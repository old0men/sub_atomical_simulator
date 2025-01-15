use bevy::prelude::*;
use bevy::{color::palettes::css::{GREEN}};

use physical_constants;
use crate::{Particle, Movement};
use crate::movement::direction_check;
pub fn gravity(
    mut query: Query<(&Transform, &mut Particle, &mut Movement), (With<Particle>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos
){
    pub const  FORCE_GRAVITY: f32 = 6.67430e-11f32;

    let mut combinations = query.iter_combinations_mut();


    while let Some([mut particle1, mut particle2]) = combinations.fetch_next() {


        let distance = ((particle2.0.translation.x - particle1.0.translation.x).powf(2.0)+(particle2.0.translation.y - particle1.0.translation.y).powf(2.0)).sqrt();

        // this is for debuging to see the range of a subatomic particle
        // the current 250.0 need to be the same as if check lower down
        /*if particle1.1.mass == 1.0 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(Color::from(ORANGE))),
                Transform::from_xyz(particle1.0.translation.x, particle1.0.translation.y, -1.0)
                    .with_scale(Vec3::splat(250.0)),
                GlobalTransform::default(),
                Movement {
                    speed: Vec2::new(particle1.2.speed.x, particle1.2.speed.y),
                    acceleration: Vec2::ZERO,
                    direction: Vec2::new(1.0, 1.0)
                },
            ));
            particle1.1.mass += 0.1;
        }*/

        if distance < 250.0
        {
            //println!("{distance}");


            let direction_particle1 = Vec3::new((particle2.0.translation.x-particle1.0.translation.x)/distance,
                                                   (particle2.0.translation.y-particle1.0.translation.y)/distance,
                                                   0.0);

            let direction_particle2 = Vec3::new((particle1.0.translation.x-particle2.0.translation.x)/distance,
                                                (particle1.0.translation.y-particle2.0.translation.y)/distance,
                                                0.0);

            let current_acceleration_change1 = Vec2::new(FORCE_GRAVITY*((particle2.1.mass/(distance*0.00003268).powf(2.0))/direction_particle1.x), FORCE_GRAVITY*((particle2.1.mass/(distance*0.00003268).powf(2.0))/direction_particle1.y));
            let current_acceleration_change2 = Vec2::new(FORCE_GRAVITY*((particle1.1.mass/(distance*0.00003268).powf(2.0))/direction_particle2.x), FORCE_GRAVITY*((particle1.1.mass/(distance*0.00003268).powf(2.0))/direction_particle2.y));



            particle1.2.acceleration.x = current_acceleration_change1.x.copysign(1.0);
            particle1.2.acceleration.y = current_acceleration_change1.y.copysign(1.0);
            particle2.2.acceleration.x = current_acceleration_change2.x.copysign(1.0);
            particle2.2.acceleration.y = current_acceleration_change2.y.copysign(1.0);;


            gizmos.line_2d(
                Vec2::new(particle2.0.translation.x, particle2.0.translation.y),
                Vec2::new(particle1.0.translation.x, particle1.0.translation.y),
                GREEN
            )
        }
    }
}