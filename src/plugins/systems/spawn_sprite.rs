use std::path::Path;

use bevy::prelude::*;

use crate::plugins::shp::shp_reader;

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<Handle<Image>>,
    pub current: usize,
    pub timer: Timer,
}

fn spawn_sprite(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    shp_path: &Path,
    pal_path: &Path,
) {
    match shp_reader::decode_shp_to_image(images, shp_path, pal_path) {
        Ok(handles) => {
            if handles.is_empty() {
                println!("No frames loaded");
                return;
            }

            commands.spawn((
                Sprite::from_image(handles[0].clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Animation {
                    frames: handles,
                    current: 0,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
            ));
        }
        Err(e) => {
            println!("Failed to load shp: {}", e);
        }
    }
}

pub fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Sprite, &mut Animation)>) {
    for (mut sprite, mut anim) in &mut query {
        // tick the timer
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            // move to the next frame
            anim.current = (anim.current + 1) % anim.frames.len();
            sprite.image = anim.frames[anim.current].clone();
        }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    spawn_sprite(
        &mut commands,
        &mut images,
        Path::new("assets/shp/numislmk.shp"),
        Path::new("assets/pal/uniturb.pal"),
    );
    setup_camera(commands);
}
