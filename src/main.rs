use crate::Direction::{Down, Left, Right, Up};
use crate::Player::{Player01, Player02};
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy::time::common_conditions::on_fixed_timer;
use bevy::window::PresentMode;
use bevy::window::WindowTheme;
use bevy::DefaultPlugins;
use rand::Rng;
use std::ptr::null_mut;

use std::time::Duration;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 700.;
const GRID_SIZE: i32 = 28;
const GRID_SQUARE_SIZE: f32 = WINDOW_WIDTH / GRID_SIZE as f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (800., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_systems(
            Update,
            (
                handle_movement,
                spawn_food.run_if(is_not_food),
                handle_eat_food.after(handle_movement),
                check_for_death.after(handle_movement),
            )
                .run_if(on_fixed_timer(Duration::from_millis(250))),
        )
        .add_systems(PostUpdate, (position_translation, handle_movement_input_p1))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.08, 0.04)))
        .run();
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    Player01,
    Player02,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
    player: Player,
}

#[derive(Component)]
struct SnakeSegment {
    player: Player,
}

#[derive(Component)]
struct Food;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // spawn p1
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::MIDNIGHT_BLUE,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
            ..default()
        },
        SnakeHead {
            direction: Up,
            player: Player01,
        },
        Position { x: 1, y: 1 },
    ));

    // spawn p2
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
            ..default()
        },
        SnakeHead {
            direction: Up,
            player: Player02,
        },
        Position { x: 26, y: 1 },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GREEN,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
            ..default()
        },
        SnakeSegment { player: Player01 },
        Position { x: 0, y: -1 },
    ));
}

fn spawn_food(mut commands: Commands) {
    let x = rand::thread_rng().gen_range(0..GRID_SIZE);
    let y = rand::thread_rng().gen_range(0..GRID_SIZE);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
            ..default()
        },
        Food,
        Position { x, y },
    ));
}

fn is_not_food(query: Query<&Food>) -> bool {
    let count: usize = query.iter().count();
    if count > 0 {
        return false;
    }
    return true;
}

fn handle_movement_input_p1(keys: Res<Input<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    let mut q = query.iter_mut();
    
    let mut head1 = q.next().unwrap();

    if keys.pressed(KeyCode::Up) && head1.direction != Down {
        head1.direction = Up;
    } else if keys.pressed(KeyCode::Down) && head1.direction != Up {
        head1.direction = Down;
    } else if keys.pressed(KeyCode::Left) && head1.direction != Right {
        head1.direction = Left;
    } else if keys.pressed(KeyCode::Right) && head1.direction != Left {
        head1.direction = Right;
    }

    let mut head2 = q.next().unwrap();

    if keys.pressed(KeyCode::W) && head2.direction != Down {
        head2.direction = Up;
    } else if keys.pressed(KeyCode::S) && head2.direction != Up {
        head2.direction = Down;
    } else if keys.pressed(KeyCode::A) && head2.direction != Right {
        head2.direction = Left;
    } else if keys.pressed(KeyCode::D) && head2.direction != Left {
        head2.direction = Right;
    }
}


fn handle_movement(
    mut query: Query<
        (&mut SnakeHead, &mut Position),
        (With<SnakeHead>, Without<SnakeSegment>),
    >,
    mut segment_query: Query<(&mut Position, &SnakeSegment), (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    for q in query.iter_mut() {
        let head = q.0;
        let mut pos = q.1;

        let prev_transform = pos.clone();

        match head.direction {
            Up => {
                pos.y += 1;
                if pos.y >= GRID_SIZE {
                    pos.y = 0;
                }
            }
            Down => {
                pos.y -= 1;
                if pos.y < 0 {
                    pos.y = GRID_SIZE - 1;
                }
            }
            Left => {
                pos.x -= 1;
                if pos.x < 0 {
                    pos.x = GRID_SIZE - 1;
                }
            }
            Right => {
                pos.x += 1;
                if pos.x >= GRID_SIZE {
                    pos.x = 0;
                }
            }
        }

        let mut prev_translation = prev_transform;
        for (mut seg_position, segment) in segment_query.iter_mut() {
            if head.player == segment.player {
                let prev = seg_position.clone();
                seg_position.x = prev_translation.x;
                seg_position.y = prev_translation.y;

                prev_translation = prev;
            }
        }
    }
}

fn handle_eat_food(
    mut commands: Commands,
    head_query: Query<(&SnakeHead, &Position), With<SnakeHead>>,
    food_query: Query<(Entity, &Position), With<Food>>,
) {
    for (head, pos) in head_query.iter() {
        for food in food_query.iter() {
            if pos.x == food.1.x && pos.y == food.1.y {
                commands.entity(food.0).despawn();
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            ..default()
                        },
                        transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                        ..default()
                    },
                    SnakeSegment {
                        player: head.player,
                    },
                    Position { x: -1, y: -1 },
                ));
            }
        }
    }
}

fn check_for_death(
    mut commands: Commands,
    entity_query: Query<(Entity, &SnakeSegment), With<Position>>,
    head_query: Query<(&Position, &SnakeHead), With<SnakeHead>>,
    segments_query: Query<&Position, With<SnakeSegment>>,
) {
    for (head, snake_head) in head_query.iter() {
        for segment in segments_query.iter() {
            if head.x == segment.x && head.y == segment.y {
                for (entity, segment) in entity_query.iter() {
                    if segment.player == snake_head.player {
                        commands.entity(entity).despawn();
                    }
                }
        }
    }
}


fn position_translation(windows: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_SIZE as f32),
            convert(pos.y as f32, window.height() as f32, GRID_SIZE as f32),
            0.0,
        );
    }
}
