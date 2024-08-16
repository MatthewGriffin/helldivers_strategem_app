use bevy::{
    asset::AssetServer,
    color::{Alpha, Color},
    math::Vec2,
    prelude::{default, BuildChildren, Camera2dBundle, Commands, NodeBundle, Res, Transform},
    sprite::{
        Anchor, BorderRect, ImageScaleMode, SliceScaleMode, Sprite, SpriteBundle, TextureSlicer,
    },
    ui::{BackgroundColor, Display, GridTrack, Style, Val},
};

pub fn setup_interface(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::Center,
                custom_size: Some(Vec2::new(1000.0, 1000.0)),

                color: Color::linear_rgba(255.0, 255.0, 255.0, 0.01),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("super_earth.png"),
            ..default()
        },
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(200.0),
            center_scale_mode: SliceScaleMode::Stretch,
            ..default()
        }),
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(0.1),
                    GridTrack::flex(0.05),
                    GridTrack::flex(0.7),
                    GridTrack::flex(0.05),
                    GridTrack::flex(0.1),
                ],
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgb(255.0, 255.0, 255.0)),
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::linear_rgb(255.0, 255.0, 255.0)),
                ..default()
            });
        });
}
