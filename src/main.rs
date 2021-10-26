use bevy::prelude::*;
use rand::prelude::random;
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

struct Segment;

#[derive(Default)]
struct Segments(Vec<Entity>);

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
        .insert_resource(Segments::default())
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
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(spawn_food.system())
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut segments: ResMut<Segments>) {

    segments.0 = vec![
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
        })
        .insert(Segment)
        .id(),
        spawn_segment(commands, materials, Location {x:8, y:7})
    ];

    
}

fn spawn_segment(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, location: Location) -> Entity {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        sprite: Sprite::new(Vec2::new(CELLSIZE, CELLSIZE)),
        ..Default::default()
    })
    .insert(Segment)
    .insert(location)
    .id()
}

fn spawn_food(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.0,1.0,0.0).into()),
        transform: Transform::from_xyz(0.0,0.0,0.0),
        sprite: Sprite::new(Vec2::new(CELLSIZE, CELLSIZE)),
        ..Default::default()
    })
    .insert(Location {
        x: (random::<f32>() * GRID_SIZE as f32) as i32,
        y: (random::<f32>() * GRID_SIZE as f32) as i32,
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

fn move_head(segments: ResMut<Segments>, mut heads: Query<(Entity, &Head)>, mut locations: Query<&mut Location>) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let locs = segments.0.iter().map(|segment| *locations.get_mut(*segment).unwrap()).collect::<Vec<Location>>();

        let mut head_loc = locations.get_mut(head_entity).unwrap();
        match head.direction {
            Direction::Up => head_loc.y = (head_loc.y + 1).min(15),
            Direction::Left => head_loc.x = (head_loc.x - 1).max(0),
            Direction::Down => head_loc.y = (head_loc.y - 1).max(0),
            Direction::Right => head_loc.x = (head_loc.x + 1).min(15),
        }

        for (loc, segment) in locs.iter().zip(segments.0.iter().skip(1)) {
            *locations.get_mut(*segment).unwrap() = *loc;
        }

    }
}