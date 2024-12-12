use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{despawn_screen, GameState};

pub fn game_plugin(app: &mut App) {
    app.add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),))
        .add_systems(Startup, create_map)
        .add_systems(
            OnEnter(GameState::Explore),
            (spawn_player, start_exploration),
        )
        .add_systems(Update, player_movement.run_if(in_state(GameState::Explore)))
        .add_systems(OnExit(GameState::Explore), despawn_screen::<OnExploration>);
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
pub struct Player(f32);

#[derive(Component)]
struct OnExploration;

#[derive(Resource)]
struct ExplorationMap {
    // 1000x1000
    tiles: Vec<[Tile; 1000]>,
}

impl ExplorationMap {
    fn from_image(image: &Image) -> ExplorationMap {
        assert!(image.width() == 1000 && image.height() == 1000);

        let mut tiles = Vec::new();

        for i in 0..1000 {
            let mut next_row = [Tile::Error; 1000];
            for j in 0..1000 {
                let color = image.get_color_at(i, j).expect("wtf");
                next_row[i as usize] = tile_from_color(color);
            }
            tiles.push(next_row);
        }

        ExplorationMap { tiles }
    }
}

fn tile_from_color(c: Color) -> Tile {
    match u32::from_be_bytes(c.to_linear().to_u8_array()) & 0xFFFFFF {
        0xFF_FF_FF => Tile::Air,
        0xDD_DD_DD => Tile::Rock,
        0x00_00_FF => Tile::Ice,
        _ => Tile::Error,
    }
}

fn create_map(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    images: Res<Assets<Image>>,
) {
    let map: Handle<Image> = asset_server.load("map.png");
    commands.insert_resource(ExplorationMap::from_image(
        images.get(&map).as_ref().unwrap(),
    ));
}

pub fn start_exploration(commands: Commands) {}

#[derive(Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Error = 0,
    Rock,
    Ice,
    Oil,
    Iron,
    Air,
}
// fn load_map(texture: Handle<Image>) {
//     let map_pixel texture::get_pixel();//server.load("mascot.png")
//     let mut map = [[Some(Tile::Rock); 8]; 8];
//     map[0][1] = None;
//}

pub fn spawn_player(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    let mut rapier_config = rapier_config.single_mut();
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;

    let sprite_size = 100.0;

    // Spawn entity with `Player` struct as a component for access in movement query.
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            image: server.load("mascot.png"),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Collider::ball(sprite_size / 2.0),
        Player(100.0),
        OnExploration,
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(500.0, 50.0),
        Transform::from_xyz(0.0, -50.0, 0.0),
        OnExploration,
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}
