use bevy::prelude::*;
use bevy::core::FixedTimestep;

const WINDOW_SIZE: f32 = 300.0;
const GRID_SIZE: u32 = 15;
const CELLSIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;

#[derive(Default, Copy, Clone, Eq, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

struct Head {
    direction: Direction,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,
    Movement,
    Eating,
    Growth,
}

fn main() {
    App::build()
    .insert_resource(WindowDescriptor {
        title: "Snake".to_string(),
        width: WINDOW_SIZE,
        height: WINDOW_SIZE,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(spawn.system())
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(update_location.system())
    )
    .add_system(
        change_direction.system()
        .label(Order::Input)
        .before(Order::Movement)
    )
    .add_system_set(
        SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.35))
        .with_system(move_head.system().label(Order::Movement))
    )
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        sprite: Sprite::new(Vec2::new(CELLSIZE, CELLSIZE)),
        ..Default::default()
    })
    .insert(Head {
        direction: Direction::Up,
    })
    .insert(Location {
        x: 8,
        y: 8,
    });
}

fn update_location(mut query: Query<(&Location, &mut Transform)>) {

    for (pos, mut transform) in query.iter_mut() {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        if pos.x > 8 {
            let diff: f32 = pos.x as f32 - 8.0;
            x = diff*CELLSIZE;
        } else if pos.x <= 8 {
            let diff: f32 = 8.0 - pos.x as f32;
            x = diff*(-CELLSIZE);
        }

        if pos.y > 8 {
            let diff: f32 = pos.y as f32 - 8.0;
            y = diff*CELLSIZE;
        } else if pos.y <= 8 {
            let diff: f32 = 8.0 - pos.y as f32;
            y = diff*(-CELLSIZE);
        }

        let winlimit = WINDOW_SIZE/2.0 - CELLSIZE/2.0;

        x = x.min(winlimit).max(-winlimit);
        y = y.min(winlimit).max(-winlimit);

        transform.translation = Vec3::new(
            x,
            y,
            0.0,
        );
    }
}

fn change_direction(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Head>) {
    
    if let Some(mut head) = query.iter_mut().next() {
        let dir: Direction = if keyboard.pressed(KeyCode::Up) {
            Direction::Up
        }
        else if keyboard.pressed(KeyCode::Left) {
            Direction::Left
        }
        else if keyboard.pressed(KeyCode::Down) {
            Direction::Down
        }
        else if keyboard.pressed(KeyCode::Right) {
            Direction::Right
        }
        else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn move_head(mut query: Query<(&mut Location, &Head)>) {
    if let Some((mut location, head)) = query.iter_mut().next() {
        // TODO: max and min the location.y and location.x so they dont go over 15 and below 0
        match &head.direction {
            Direction::Up => location.y += 1,
            Direction::Left => location.x -= 1,
            Direction::Down => location.y -= 1,
            Direction::Right => location.x +=1,
        }
    }
}