//enum state {
//    talking: usize,
//    picking: Vec<option>,
//}

#[derive(Copy, Clone, Debug)]
enum MissionType {
    Water,
    Explore,
    Oil,
    Iron,
}

enum CharactersType {
    JanitorJoe,
    Oldlady,
    Twin1,
    Twin2,
    Cat,
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
}

struct DialogueOption {
    scene_flag: usize,
    mission: Option<MissionType>,
}

use bevy::{math::ops, prelude::*, window::PrimaryWindow};

use crate::GameState;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(
//             Update,
//             (
//                 animate_translation,
//                 animate_rotation,
//                 animate_scale,
//                 follow_mouse,
//             ),
//         )
//         .run();
//}

#[derive(Component)]
struct FollowsMouse;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

pub fn dating_sim_plugin(app: &mut App) {
    let janitor_joe = CharactersStatus {
        character: CharactersType::JanitorJoe,
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
            scene_flag: 2,
            mission: None,
        },
        favor: 20,
        alive: true,
    };

    let granny = CharactersStatus {
        character: CharactersType::Oldlady,
        current_dialogue: DialogueOption {
            scene_flag: 3,
            mission: Some(MissionType::Oil),
        },
        favor: 20,
        alive: true,
    };

    let characters = vec![janitor_joe, granny, cat];

    app.insert_resource(DatingContext {
        all_characters: characters,
    });

    app.add_systems(OnEnter(GameState::DatingSim), on_dating_sim);
}

fn on_dating_sim(
    mut commands: Commands,
    context: ResMut<DatingContext>,
    asset_server: Res<AssetServer>,
) {
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
    for (idx, i) in context.all_characters.iter().enumerate() {
        let box_position = dbg!(Vec2::new((idx as f32 * 200.0) - 500.0, 250.0));
        if let Some(mission_var) = i.current_dialogue.mission {
            let box_size = Vec2::new(100.0, 100.0);
            let box_position = box_position + Vec2::new(0.0, -150.0);
            let enc = commands.spawn((
                Sprite::from_color(Color::srgb(0.75, 0.25, 0.25), box_size),
                Transform::from_translation(box_position.extend(0.0)),
            ));
            //if (idx == 0) {
            //    enc.insert(FollowsMouse);
            //}
        }

        let box_size = Vec2::new(150.0, 150.0);
        commands.spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_size),
            Transform::from_translation(box_position.extend(0.0)),
        ));
        //.with_children(|builder| {
        //    builder.spawn((
        //        Text2d::new(i.text.clone()),
        //        slightly_smaller_text_font.clone(),
        //        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        //        // Wrap text in the rectangle
        //        TextBounds::from(box_size),
        //        // ensure the text is drawn on top of the box
        //        Transform::from_translation(Vec3::Z),
        //    ));
        //});
    }

    let text_justification = JustifyText::Center;
    // 2d camera
    commands.spawn(Camera2d);
    // Demonstrate changing translation
}

fn off_dating_sim() {}

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

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        let scale = (ops::sin(time.elapsed_secs()) + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
