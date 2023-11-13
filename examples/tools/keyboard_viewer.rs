use bevy::prelude::*;

const KEY_SIZE: Val = Val::Px(80.);

/// update the border color of every key on the on-screen keyboard to show if it's pressed.
/// the most interesting system in this example so I put it first
fn update_keyboard(
    //
    keyboard: Res<Input<KeyCode>>,
    mut icons: Query<(&Key, &mut BorderColor)>,
) {
    if keyboard.is_changed() {
        let held_keys: Vec<_> = keyboard.get_pressed().map(KeyCode::to_owned).collect();
        println!("{:?}", held_keys);
        for (key, mut border) in icons
            .iter_mut()
            .filter(|k| k.0.code.is_some())
            .map(|k| (k.0.code.unwrap(), k.1))
        {
            if held_keys.contains(&key) {
                *border = Color::GREEN.into();
            } else {
                *border = Color::WHITE.into();
            }
        }
    }
}

#[derive(Debug, Clone, Component)]
struct Key {
    code: Option<KeyCode>,
    display: String,
    width: f32,
}

impl Key {
    fn gap(width: f32) -> Self {
        Key {
            code: None,
            display: "".to_string(),
            width,
        }
    }
}

macro_rules! k {
    ($k: ident) => {
        Key {
            code: Some(KeyCode::$k),
            display: stringify!($k).into(),
            width: 1.,
        }
    };
    ($k: ident, $disp: literal) => {
        Key {
            code: Some(KeyCode::$k),
            display: $disp.into(),
            width: 1.,
        }
    };
    ($k: ident, $line1: literal, $line2: literal) => {
        Key {
            code: Some(KeyCode::$k),
            display: ($line1.to_string() + "\n" + $line2),
            width: 1.,
        }
    };
}

fn keyboard_layout() -> Vec<Vec<Key>> {
    vec![
        vec![
            k!(Escape, "ESC"),
            Key::gap(1.75 / 3.),
            k!(F1),
            k!(F2),
            k!(F3),
            k!(F4),
            Key::gap(1.75 / 3.),
            k!(F5),
            k!(F6),
            k!(F7),
            k!(F8),
            Key::gap(1.75 / 3.),
            k!(F9),
            k!(F10),
            k!(F11),
            k!(F12),
        ],
        vec![
            k!(Grave, "~", "`"),
            k!(Key1, "!", "1"),
            k!(Key2, "@", "2"),
            k!(Key3, "#", "3"),
            k!(Key4, "$", "4"),
            k!(Key5, "%", "5"),
            k!(Key6, "^", "6"),
            k!(Key7, "&", "7"),
            k!(Key8, "*", "8"),
            k!(Key9, "(", "9"),
            k!(Key0, ")", "0"),
            k!(Minus, "_", "-"),
            k!(Equals, "+", "="),
            Key {
                code: Some(KeyCode::Back),
                display: "|<-".into(),
                width: 1.75,
            },
        ],
        vec![
            Key {
                code: Some(KeyCode::Tab),
                display: "Tab".into(),
                width: 1.5,
            },
            k!(Q),
            k!(W),
            k!(E),
            k!(R),
            k!(T),
            k!(Y),
            k!(U),
            k!(I),
            k!(O),
            k!(P),
            k!(BracketLeft, "{", "["),
            k!(BracketRight, "}", "]"),
            Key {
                code: Some(KeyCode::Backslash),
                display: "|\n\\".into(),
                width: 1.25,
            },
        ],
        vec![
            Key {
                code: Some(KeyCode::Capital),
                display: "Caps\nLock".into(),
                width: 1.75,
            },
            k!(A),
            k!(S),
            k!(D),
            k!(F),
            k!(G),
            k!(H),
            k!(J),
            k!(K),
            k!(L),
            k!(Semicolon, ";"),
            k!(Apostrophe, "\"", "'"),
            Key {
                code: Some(KeyCode::Return),
                display: "Enter".into(),
                width: 2.,
            },
        ],
        vec![
            Key {
                code: Some(KeyCode::ShiftLeft),
                display: "Left\nShift".into(),
                width: 2.25,
            },
            k!(Z),
            k!(X),
            k!(C),
            k!(V),
            k!(B),
            k!(N),
            k!(M),
            k!(Comma, ","),
            k!(Period, "."),
            k!(Slash, "?", "/"),
            Key {
                code: Some(KeyCode::ShiftRight),
                display: "Right\nShift".into(),
                width: 2.5,
            },
        ],
        vec![
            Key {
                code: Some(KeyCode::ControlLeft),
                display: "Ctrl".into(),
                width: 1.5,
            },
            Key {
                code: Some(KeyCode::SuperLeft),
                display: "Super".into(),
                width: 1.25,
            },
            Key {
                code: Some(KeyCode::AltLeft),
                display: "Alt".into(),
                width: 1.25,
            },
            Key {
                code: Some(KeyCode::Space),
                display: "Space".into(),
                width: 7.,
            },
            Key {
                code: Some(KeyCode::AltRight),
                display: "Alt".into(),
                width: 1.25,
            },
            Key {
                code: Some(KeyCode::SuperRight),
                display: "Super".into(),
                width: 1.25,
            },
            Key {
                code: Some(KeyCode::ControlRight),
                display: "Ctrl".into(),
                width: 1.25,
            },
        ],
    ]
}

fn setup(mut commands: Commands) {
    let layout = keyboard_layout();
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for row in layout.iter() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: KEY_SIZE,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for key in row.iter() {
                            let border: UiRect = match key.code {
                                Some(_) => UiRect::all(Val::Px(7.)),
                                None => UiRect::default(),
                            };
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: KEY_SIZE * key.width,
                                        height: KEY_SIZE,
                                        border,
                                        ..default()
                                    },
                                    border_color: Color::WHITE.into(),
                                    ..default()
                                })
                                .insert(key.clone())
                                .with_children(|parent| {
                                    parent.spawn((TextBundle {
                                        text: Text::from_section(
                                            key.display.clone(),
                                            TextStyle {
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.)),
                                            padding: UiRect::all(Val::Px(10.)),
                                            ..default()
                                        },
                                        ..default()
                                    },));
                                });
                        }
                    });
            }
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_keyboard)
        .run();
}
