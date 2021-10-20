use bevy::prelude::*;

struct Head {
    direction: i64,
    timer: f32
}

fn main() {
    App::build()
    .insert_resource(WindowDescriptor {
        title: "Snake".to_string(),
        width: 300.0,
        height: 300.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(spawn.system())
    .add_system(change_direction.system())
    .add_system(move_head.system())
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        sprite: Sprite::new(Vec2::new(20.0,20.0)),
        ..Default::default()
    }).insert(Head {
        direction: 0,
        timer: 0.0
    });
}

fn change_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Head>) {
    for mut head in query.iter_mut() {

        if keyboard_input.pressed(KeyCode::Up) {
            head.direction = 1;
        }

        else if keyboard_input.pressed(KeyCode::Right) {
            head.direction = 2;
        }

        else if keyboard_input.pressed(KeyCode::Down) {
            head.direction = 3;
        }

        else if keyboard_input.pressed(KeyCode::Left) {
            head.direction = 4;
        }
    }
}

fn move_head(time: Res<Time>, mut query: Query<(&mut Head, &mut Transform)>) {
    let thershold: f32 = 0.2;

    for (mut head, mut transform) in query.iter_mut() {
        head.timer += time.delta_seconds();

        if head.timer >= thershold {
            let translation = &mut transform.translation;

            if head.direction == 1 {
                translation.y += 20.0
            }

            else if head.direction == 2 {
                translation.x += 20.0
            }

            else if head.direction == 3 {
                translation.y -= 20.0
            }

            else if head.direction == 4 {
                translation.x -= 20.0
            }

            translation.x = translation.x.min(140.0).max(-140.0);
            translation.y = translation.y.min(140.0).max(-140.0);

            head.timer = 0.0;
        }
    }
}