use bevy::prelude::*;
use super::components::*;
use super::styles::*;
use crate::editor::resources::EditorConfig;

pub fn setup_editor_ui(mut commands: Commands) {
    // Root Node (Overlay)
    commands.spawn((
        Node {
             width: Val::Percent(100.0),
             height: Val::Percent(100.0),
             justify_content: JustifyContent::SpaceBetween,
             ..default()
        },
        EditorRoot,
        Pickable::IGNORE, // Don't block picking for the scene unless hitting UI
    ))
    .with_children(|parent| {
        // Left Panel (Hierarchy)
        parent.spawn((
            sidebar_style(),
            BackgroundColor(PANEL_COLOR),
            HierarchyPanel,
        )).with_children(|p| {
            p.spawn((
                Text::new("Hierarchy"),
                TextFont::default(),
                TextColor(HEADER_COLOR),
                BackgroundColor(TEXT_COLOR) // Invert for header bg?
            ));
            // List container
            p.spawn(Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..default()
            });
        });

        // Viewport (Invisible placeholder for scene interaction area)
        parent.spawn((
            viewport_style(),
            ViewportPanel,
        ));

        // Right Panel (Inspector)
        parent.spawn((
            sidebar_style(),
            BackgroundColor(PANEL_COLOR),
            InspectorPanel,
        )).with_children(|p| {
             p.spawn((
                Text::new("Inspector"),
            ));
        });
    });
}

pub fn toggle_editor(
    input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<EditorConfig>,
    mut query: Query<&mut Visibility, With<EditorRoot>>,
) {
    if input.just_pressed(KeyCode::F1) {
        config.show_editor = !config.show_editor;
        for mut vis in query.iter_mut() {
            *vis = if config.show_editor { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}
