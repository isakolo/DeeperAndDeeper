//enum state {
//    talking: usize,
//    picking: Vec<option>,
//}

struct choice {
    text: String,
    upcomin_dialog: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, dialogSetup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn dialogSetup() {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;
    // 2d camera
    commands.spawn(Camera2d);
    // Demonstrate changing translation
    // Demonstrate text wrapping
    let slightly_smaller_text_font = TextFont {
        font,
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.25, 0.25, 0.75), box_size),
            Transform::from_translation(box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in the box\n(Unicode linebreaks)"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                // Wrap text in the rectangle
                TextBounds::from(box_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgb(0.20, 0.3, 0.70), other_box_size),
            Transform::from_translation(other_box_position.extend(0.0)),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new("this text wraps in the box\n(AnyCharacter linebreaks)"),
                slightly_smaller_text_font.clone(),
                TextLayout::new(JustifyText::Left, LineBreak::AnyCharacter),
                // Wrap text in the rectangle
                TextBounds::from(other_box_size),
                // ensure the text is drawn on top of the box
                Transform::from_translation(Vec3::Z),
            ));
        });
}
