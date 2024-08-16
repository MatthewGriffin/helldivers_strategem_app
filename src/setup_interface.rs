use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{default, BuildChildren, Camera2dBundle, Commands, NodeBundle, Res},
    ui::{BackgroundColor, Display, GridTrack, Style, UiImage, Val},
};

pub fn setup_interface(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

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

            parent
                .spawn(NodeBundle {
                    style: Style {
                        left: Val::Percent(25.0),
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        UiImage {
                            texture: asset_server.load("super_earth.png"),
                            color: Color::linear_rgba(255.0, 255.0, 255.0, 0.01),
                            ..default()
                        },
                    ));
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
