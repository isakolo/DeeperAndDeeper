//enum state {
//    talking: usize,
//    picking: Vec<option>,
//}

enum MissionType {
    water,
    explore,
    oil,
    iron,
}

struct DialogueOption {
    text: String,
    scene_Flag: usize,
    mission: Option<MissionType>,
}

use bevy::{
    color::palettes::css::*,
    math::ops,
    prelude::*,
    sprite::Anchor,
    text::{FontSmoothing, LineBreak, TextBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, animate_rotation, animate_scale),
        )
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    let dialouge_GETIRON = DialogueOption {
        text: String::from("I need iron"),
        scene_Flag: 1,
        mission: Some(MissionType::iron),
    };
    let dialouge_GETWATER = DialogueOption {
        text: String::from("I'm thirsty"),
        scene_Flag: 2,
        mission: Some(MissionType::water),
    };
    let dialouge_GETOIL = DialogueOption {
        text: String::from("GIVE ME OIL"),
        scene_Flag: 3,
        mission: Some(MissionType::oil),
    };

    let dialouge_TalkShit = DialogueOption {
        text: String::from("Talk shit with Joe"),
        scene_Flag: 3,
        mission: None,
    };

    let dialogue_options = vec![
        dialouge_GETIRON,
        dialouge_GETWATER,
        dialouge_GETOIL,
        dialouge_TalkShit,
    ];

    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };
    for (idx, i) in dialogue_options.iter().enumerate() {
        let box_size = Vec2::new(800.0, 100.0);
        let box_position = dbg!(Vec2::new(-100.0, (idx as f32 * -125.0) + 250.0));
        commands
            .spawn((
                Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_size),
                Transform::from_translation(box_position.extend(0.0)),
            ))
            .with_children(|builder| {
                builder.spawn((
                    Text2d::new(i.text.clone()),
                    slightly_smaller_text_font.clone(),
                    TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                    // Wrap text in the rectangle
                    TextBounds::from(box_size),
                    // ensure the text is drawn on top of the box
                    Transform::from_translation(Vec3::Z),
                ));
            });
    }

    let text_justification = JustifyText::Center;
    // 2d camera
    commands.spawn(Camera2d);
    // Demonstrate changing translation
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
