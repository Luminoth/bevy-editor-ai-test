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
             flex_direction: FlexDirection::Column,
             ..default()
        },
        EditorRoot,
        Pickable::IGNORE, // Don't block picking for the scene unless hitting UI
    ))
    .with_children(|parent| {
        // Menu Bar
        // Inline menu setup to avoid ChildBuilder type naming issues
        parent.spawn((
            menu_bar_style(),
            MenuBar,
            BackgroundColor(PANEL_COLOR),
        )).with_children(|menu| {
            // File Button
            menu.spawn((
                Button,
                menu_button_style(),
                BackgroundColor(BUTTON_COLOR_NORMAL),
                FileMenuButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("File"),
                    TextFont::default(),
                    TextColor(TEXT_COLOR),
                ));
            });

            // Dropdown Logic (Initially Hidden)
            menu.spawn((
                dropdown_style(),
                BackgroundColor(PANEL_COLOR),
                FileMenuDropdown,
                Visibility::Hidden,
                GlobalZIndex(10), // Ensure it's on top
            )).with_children(|dropdown| {
                // Save
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Save },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Save"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Save As
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::SaveAs },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Save As"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Load
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Load },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Load"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });

                // Exit
                dropdown.spawn((
                    Button,
                    menu_button_style(),
                    BackgroundColor(BUTTON_COLOR_NORMAL),
                    MenuButtonAction { action: MenuAction::Exit },
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Exit"),
                        TextFont::default(),
                        TextColor(TEXT_COLOR),
                    ));
                });
            });
        });

        // Main Content Area
        parent.spawn(Node {
             width: Val::Percent(100.0),
             flex_grow: 1.0,
             flex_direction: FlexDirection::Row,
             justify_content: JustifyContent::SpaceBetween,
             ..default()
        }).with_children(|main_parent| {
             // Left Panel (Hierarchy)
             main_parent.spawn((
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
             main_parent.spawn((
                 viewport_style(),
                 ViewportPanel,
             ));

             // Right Panel (Inspector)
             main_parent.spawn((
                 sidebar_style(),
                 BackgroundColor(PANEL_COLOR),
                 InspectorPanel,
             )).with_children(|p| {
                  p.spawn((
                     Text::new("Inspector"),
                 ));
             });
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
