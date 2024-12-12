//enum state {
//    talking: usize,
//    picking: Vec<option>,
//}

use super::{despawn_screen, GameState};
use crate::load;
use bevy::{
    math::ops,
    prelude::*,
    text::{FontSmoothing, LineBreak, TextBounds},
    window::PrimaryWindow,
};
use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug)]
enum MissionType {
    Water,
    Explore,
    Oil,
    Iron,
}

#[derive(Deserialize, Debug)]
enum CharactersType {
    Joe,
    Jule,
    Carle,
    Fredrick,
    Diedrick,
    Cat,
    Liv,
    Main,
}

struct CharactersStatus {
    character: CharactersType,
    current_dialogue: DialogueOption,
    favor: usize,
    alive: bool,
}

#[derive(Resource)]
struct DatingContext {
    all_characters: Vec<CharactersStatus>,
    day: usize,
    cursor: isize,
    selected_scene: DatingScene,
    flags: Vec<(String, isize)>,
    gathered_mission: Vec<MissionType>,
}

struct DialogueOption {
    scene_flag: usize,
    mission: Option<MissionType>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum DatingState {
    #[default]
    Chilling,
    Talking,
    Choosing,
}

#[derive(Deserialize, Debug)]
pub struct DatingScene {
    id: String,
    person: Option<CharactersType>,
    text: Vec<String>,
    outcome: Option<Vec<(String, isize)>>,
    choice: Option<((String, String), (String, String))>,
    mission: Option<MissionType>,
}

#[derive(Component)]
struct FollowsMouse;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

#[derive(Component)]
struct Cursor(isize);

#[derive(Component)]
struct Portrait;

#[derive(Component)]
struct MissionNot;

#[derive(Component)]
struct DatingObj;

#[derive(Component)]
struct TalkObj;

#[derive(Component)]
struct TextBox(usize);

pub fn dating_sim_plugin(app: &mut App) {
    let _ = load::load_scenes();

    let janitor_joe = CharactersStatus {
        character: CharactersType::Joe,
        current_dialogue: DialogueOption {
            scene_flag: 2,
            mission: Some(MissionType::Water),
        },
        favor: 20,
        alive: true,
    };

    let cat = CharactersStatus {
        character: CharactersType::Cat,
        current_dialogue: DialogueOption {
            scene_flag: 8,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let granny = CharactersStatus {
        character: CharactersType::Jule,
        current_dialogue: DialogueOption {
            scene_flag: 3,
            mission: Some(MissionType::Oil),
        },
        favor: 20,
        alive: true,
    };

    let twin1 = CharactersStatus {
        character: CharactersType::Frederick,
        current_dialogue: DialogueOption {
            scene_flag: 4,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let twin2 = CharactersStatus {
        character: CharactersType::Diedrick,
        current_dialogue: DialogueOption {
            scene_flag: 4,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let carly = CharactersStatus {
        character: CharactersType::Carly,
        current_dialogue: DialogueOption {
            scene_flag: 4,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let liv = CharactersStatus {
        character: CharactersType::Liv,
        current_dialogue: DialogueOption {
            scene_flag: 4,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let characters = vec![janitor_joe, granny, cat, twin1, twin2, carly, liv];

    app.insert_resource(DatingContext {
        all_characters: characters,
        day: 1,
        cursor: 2,
        selected_scene: DatingScene {
            id: "1".to_string(),
            text: vec![
                "This is a placeholder".to_string(),
                "This is a second placeholder".to_string(),
            ],
            person: None,
            outcome: None,
            choice: None,
            mission: None,
        },
        flags: vec![],
        gathered_mission: vec![],
    });

    app.add_systems(OnEnter(GameState::DatingSim), on_dating_sim)
        .add_systems(Update, cursor_action.run_if(in_state(GameState::DatingSim)))
        .add_systems(OnExit(GameState::DatingSim), despawn_screen::<DatingObj>);

    app.init_state::<DatingState>();

    app.add_systems(OnEnter(DatingState::Talking), start_talking)
        .add_systems(
            Update,
            talking_action.run_if(in_state(DatingState::Talking)),
        )
        .add_systems(OnExit(DatingState::Talking), despawn_screen::<TalkObj>);

    app.add_systems(
        OnExit(DatingState::Chilling),
        (despawn_screen::<Portrait>, despawn_screen::<MissionNot>),
    );
}

fn on_dating_sim(
    mut commands: Commands,
    context: ResMut<DatingContext>,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };

    //Cursor initialisation

    let background_size = Some(Vec2::new(width, height));
    let background_position = Vec2::new(0.0, 0.0);
    let enc = commands.spawn((
        Sprite {
            image: asset_server.load("Backgrounds/deeper_deeper_base.png"),
            custom_size: background_size,
            ..Default::default()
        },
        Transform::from_translation(background_position.extend(-1.0)),
        DatingObj,
    ));

    let cursor_size = Vec2::new(width / 10.0, width / 10.0);
    let cursor_position = Vec2::new(0.0, 0.0);
    let enc = commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.75, 0.25), cursor_size),
        Transform::from_translation(cursor_position.extend(0.0)),
        Cursor(0),
        Portrait,
        DatingObj,
    ));

    for (idx, i) in context.all_characters.iter().enumerate() {
        let size = width / 9.0;
        let portrait = match i.character {
            CharactersType::Joe => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Janitor Joe-Recovered.png"),
                ..Default::default()
            },
            CharactersType::Jule => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_General_Jule.png"),
                ..Default::default()
            },
            CharactersType::Frederick => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_Twin_Dedrick.png"),
                ..Default::default()
            },

            CharactersType::Diedrick => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_Twin_Fredrick.png"),
                ..Default::default()
            },

            CharactersType::Carly => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_Carly.png"),
                ..Default::default()
            },
            CharactersType::Liv => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_Liv.png"),
                ..Default::default()
            },
            CharactersType::Cat => Sprite {
                custom_size: Some(Vec2::new(size, size)),
                image: asset_server.load("Portraits/Character_cat.png"),
                ..Default::default()
            },
            _ => Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), Vec2::new(size, size)),
        };

        let box_position = dbg!(Vec2::new((idx as f32 * size * 1.2) - width / 2.5, 250.0));
        if let Some(mission_var) = i.current_dialogue.mission {
            let box_size = Vec2::new(size / 1.5, size / 1.5);
            let box_position = box_position + Vec2::new(0.0, -150.0);
            let enc = commands.spawn((
                Sprite::from_color(Color::srgb(0.75, 0.25, 0.25), box_size),
                Transform::from_translation(box_position.extend(0.0)),
                DatingObj,
                MissionNot,
            ));
        };

        let box_size = Vec2::new(size, size);
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.75, 0.75, 0.75), box_size),
                Transform::from_translation(box_position.extend(0.0)),
                Portrait,
                DatingObj,
            ))
            .with_children(|builder| {
                builder.spawn((portrait, Transform::from_translation(Vec3::Z)));
            });
    }

    let text_justification = JustifyText::Center;
}

fn start_talking(
    mut commands: Commands,
    context: ResMut<DatingContext>,
    mut query: Query<&mut Transform, With<Cursor>>,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };

    let talk_size = Vec2::new(width / 1.6, width / 10.0);
    let talk_position = Vec2::new(0.0, -150.0);

    let dialogue = context.selected_scene.text[0].clone();
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.20, 0.3, 0.70), talk_size),
            Transform::from_translation(talk_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(dialogue),
                TextBox(0),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                // Wrap text in the rectangle
                TextBounds::from(talk_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });
}

fn talking_action(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TextBox, &mut Text2d), With<TextBox>>,
    mut context: ResMut<DatingContext>,
    mut tmp: ResMut<NextState<DatingState>>,
) {
    let confirm = keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::KeyZ);
    let escape = keyboard_input.just_pressed(KeyCode::Escape);

    if escape {
        tmp.set(DatingState::Chilling);
    } else if confirm {
        for (mut textbox, mut text) in &mut query {
            (*textbox).0 += 1;
            if (*textbox).0 < context.selected_scene.text.len() {
                let dialogue = dbg!(context.selected_scene.text[(*textbox).0 as usize].clone());
                *text = Text2d::new(dialogue);
            } else {
                //We have finished reading
                if let Some(mission) = context.selected_scene.mission {
                    context.gathered_mission.push(mission);
                }
                if context.selected_scene.outcome.is_some() {
                    println!("Added flag, but not implemented")
                }
                if context.selected_scene.choice.is_some() {
                    todo!()
                    //context.selected_scene = Some(context.selected_scene.choice)[0][1];
                } else {
                    tmp.set(DatingState::Chilling);
                }
            }
        }
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
    }
}

fn follow_mouse(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut transform: Query<&mut Transform, With<FollowsMouse>>,
) {
    let Some(position) = q_windows.single().cursor_position() else {
        return;
    };

    for mut transform in &mut transform {
        transform.translation = position.extend(0.0);
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
    }
}

fn cursor_action(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Cursor>>,
    mut context: ResMut<DatingContext>,
    mut tmp: ResMut<NextState<DatingState>>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.

    let left = keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft);
    let right = keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight);
    let confirm = keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::KeyZ);

    if confirm {
        tmp.set(DatingState::Talking);
    }

    context.cursor += -(left as isize) + right as isize;

    for mut transform in &mut query {
        transform.translation.x = (context.cursor * 180) as f32;
    }
}
