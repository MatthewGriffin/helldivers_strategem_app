use bevy::{
    color::Color,
    prelude::{default, BuildChildren, Camera2dBundle, Commands, NodeBundle},
    ui::{BackgroundColor, Display, GridTrack, Style, Val},
};

pub fn setup_interface(mut commands: Commands) {
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
